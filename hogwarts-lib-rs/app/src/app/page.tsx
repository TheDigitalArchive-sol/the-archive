"use client";

import { useState, useEffect } from "react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { useWallet } from "@solana/wallet-adapter-react";
import { Connection, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { useProvider } from "./utils";
import { Program, Idl } from "@project-serum/anchor";
import { Transaction, Message } from "@solana/web3.js";
import { useRouter } from 'next/navigation';
import { distributeRewards } from './utils/royalties';

import {
  Metaplex,
  walletAdapterIdentity
} from '@metaplex-foundation/js';

export default function Home() {
  const [isClient, setIsClient] = useState(false);
  const [wasm, setWasm] = useState<any | null>(null);
  const [anchorBridge, setAnchorBridge] = useState<any | null>(null);
  const [initResponse, setInitResponse] = useState<string | null>(null);
  const wallet = useWallet();
  const [balance, setBalance] = useState<number | null>(null);
  const [loading, setLoading] = useState(false);
  const [program, setProgram] = useState<Program<Idl> | null>(null);

  const [pdaAddress, setPdaAddress] = useState<string | null>(null);
  const [txId, setTxId] = useState<string | null>(null);
  const [bookContent, setBookContent] = useState("");
  const [retrievedContent, setRetrievedContent] = useState<string | null>(null);
  const [uploadedJson, setUploadedJson] = useState<any | null>(null);
  const router = useRouter();

  const connection = new Connection("http://127.0.0.1:8899");

  const PROGRAM_ID = process.env.NEXT_PUBLIC_PROGRAM_ID!;
  const UNSAFE_KEY = process.env.NEXT_PUBLIC_UNSAFE_KEY!;

  useEffect(() => {
    if (!PROGRAM_ID) {
      console.error("❌ Missing PROGRAM_ID!");
      return;
    }

    import("/home/rzanei/dev/the-archive/powerwand/pkg/powerwand.js").then(async (module) => {
      await module.default();
      setWasm(module);

      if (wallet.connected && wallet.publicKey) {
        try {
          console.log("✅ Connected Wallet:", wallet.publicKey.toBase58());

          const bridge = new module.AnchorBridge(wallet.publicKey.toBase58(), PROGRAM_ID);
          setAnchorBridge(bridge);
        } catch (error) {
          console.error("❌ Error using wallet as payer:", error);
        }
      }
    }).catch((error) => console.error("❌ Error loading WASM module:", error));
  }, [wallet]);

  const initializeStorageAccount = async (): Promise<string | null> => {
    if (!anchorBridge || !wallet.signTransaction || !connection) {
      console.warn("⚠️ AnchorBridge instance, wallet signer, or connection not available.");
      return null;
    }

    try {
      const totalSize = 900;
      const totalChunks = 10;

      if (!wallet.publicKey) {
        console.error("❌ Wallet is not connected!");
        return null;
      }
      const seed = `book_${Date.now().toString()}`;
      const [pda] = await PublicKey.findProgramAddress(
        [Buffer.from(seed)],
        new PublicKey(PROGRAM_ID)
      );

      console.log("📌 Unique PDA Address for New Book:", pda.toBase58());
      setPdaAddress(pda.toBase58());

      const txBase64 = await anchorBridge.initialize_storage_account(seed, totalSize, totalChunks);
      console.log("🔍 Raw WASM output:", txBase64);

      if (!txBase64 || typeof txBase64 !== "string") {
        throw new Error("Received invalid transaction data from WASM.");
      }

      const txMessageBytes = Buffer.from(txBase64, "base64");
      let reconstructedTx = Transaction.populate(Message.from(txMessageBytes));
      const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash();
      reconstructedTx.recentBlockhash = blockhash;
      reconstructedTx.lastValidBlockHeight = lastValidBlockHeight;
      const signedTransaction = await wallet.signTransaction(reconstructedTx);
      console.log("✅ Signed Transaction:", signedTransaction);

      const transactionId = await connection.sendRawTransaction(signedTransaction.serialize(), {
        skipPreflight: false,
        preflightCommitment: "confirmed",
      });
      console.log("✅ Transaction ID:", transactionId);
      setTxId(transactionId);
      setInitResponse(`Success: ${transactionId}`);
      return pda.toBase58();

    } catch (error) {
      console.error("❌ Error initializing storage account:", error);
      setInitResponse("Error initializing storage account.");
      return null;
    }
  };

  const storeDataInChunks = async (key: any, jsonData: any, targetPda: string) => {
    if (!anchorBridge || !wallet.signAllTransactions || !connection || !targetPda) {
      console.warn("⚠️ Storage account not initialized or wallet unavailable.");
      return;
    }

    try {
      if (!jsonData) {
        console.warn("⚠️ No content to store.");
        return;
      }

      console.log("📖 Preparing JSON data for encryption...");

      jsonData.publisher = wallet.publicKey?.toBase58?.() || jsonData.publisher;
      const today = new Date();
      const formattedDate = `${String(today.getDate()).padStart(2, '0')}/${String(today.getMonth() + 1).padStart(2, '0')}/${today.getFullYear()}`;

      jsonData.publication_date = jsonData.publication_date || formattedDate;
      jsonData.isbn = jsonData.isbn || anchorBridge.generate_isbn(jsonData.title, jsonData.authors);

      const jsonString = JSON.stringify(jsonData);
      let encrypted_data;
      try {
        console.log("🔍 Encrypting JSON data...");
        encrypted_data = await anchorBridge.light_msg_encryption(key, jsonString);
        console.log("✅ Encryption successful.");
        console.log("✅ Encrypted Data (Hex):", Buffer.from(encrypted_data).toString("hex"));
      } catch (error) {
        console.error("❌ WASM encryption failed:", error);
        return;
      }

      console.log(`📡 Storing encrypted data chunks...`);
      const txsBase64 = await anchorBridge.store_data_in_chunks(targetPda, encrypted_data, 900);
      console.log("🔍 Raw WASM output:", txsBase64);

      if (!Array.isArray(txsBase64) || txsBase64.length === 0) {
        throw new Error("Received invalid transaction data from WASM.");
      }

      let transactions = [];
      for (const txBase64 of txsBase64) {
        const txMessageBytes = Buffer.from(txBase64, "base64");
        let reconstructedTx = Transaction.populate(Message.from(txMessageBytes));

        const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash();
        reconstructedTx.recentBlockhash = blockhash;
        reconstructedTx.lastValidBlockHeight = lastValidBlockHeight;

        transactions.push(reconstructedTx);
      }

      console.log("🔏 Signing all transactions...");
      const signedTransactions = await wallet.signAllTransactions(transactions);
      console.log("✅ All transactions signed!");

      for (const signedTx of signedTransactions) {
        await new Promise(resolve => setTimeout(resolve, 1000));
        const txId = await connection.sendRawTransaction(signedTx.serialize(), {
          skipPreflight: false,
          preflightCommitment: "confirmed",
        });
        console.log("✅ Sent transaction:", txId);
      }

      console.log("🎉 All book content stored successfully!");

    } catch (error) {
      console.error("❌ Error storing data:", error);
    }
  };

  const handleFileUpload = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) {
      console.warn("⚠️ No file selected.");
      return;
    }

    const reader = new FileReader();
    reader.readAsText(file);

    reader.onload = async (e) => {
      try {
        const fileContent = e.target?.result as string;

        const jsonData = JSON.parse(fileContent);
        console.log("📂 JSON File Content:", jsonData);

        setUploadedJson(jsonData);
        console.log("✅ JSON File stored in state. Click 'Submit' to process it.");

      } catch (error) {
        console.error("❌ Error parsing JSON file:", error);
      }
    };
  };

  const fetchBalance = async () => {
    if (!wallet.publicKey) return;
    setLoading(true);
    try {
      const balance = await connection.getBalance(new PublicKey(wallet.publicKey));
      setBalance(balance / 1e9);
    } catch (error) {
      console.error("Error fetching balance:", error);
    }
    setLoading(false);
  };

  const provider = useProvider();
  useEffect(() => {
    if (wallet && wallet.publicKey && provider) {
      setProgram(program);
      fetchBalance();
    }
  }, [wallet]);

  const retrieveStoredData = async (key: string) => {
    if (!connection || !pdaAddress || !anchorBridge) {
      console.warn("⚠️ Connection, Storage Account PDA, or AnchorBridge not available.");
      return;
    }

    try {
      console.log("📥 Retrieving stored book data...");
      const accountInfo = await connection.getAccountInfo(new PublicKey(pdaAddress));
      if (!accountInfo || !accountInfo.data) {
        console.error("❌ No data found in storage account!");
        return;
      }

      console.log("📏 Raw Data Length:", accountInfo.data.length);

      let storedBytes = accountInfo.data.slice(20);

      while (storedBytes.length > 0 && storedBytes[storedBytes.length - 1] === 0) {
        storedBytes = storedBytes.slice(0, -1);
      }

      console.log("✅ Encrypted Data (Hex) FROM CHAIN:", Buffer.from(storedBytes).toString("hex"));
      console.log("🔍 Stored Bytes:", storedBytes);

      if (!storedBytes || storedBytes.length === 0) {
        console.error("❌ No valid data found in storage account!");
        return;
      }

      let storedText;
      try {
        storedText = await anchorBridge.light_msg_decryption(key, storedBytes);
      } catch (error) {
        console.error("❌ Decryption failed:", error);
        return;
      }

      console.log("📖 Stored Book Content:", storedText);
      setRetrievedContent(storedText);
    } catch (error) {
      console.error("❌ Error retrieving stored data:", error);
    }
  };

  const buildRoyaltyCreators = (json: any): { address: PublicKey; share: number }[] => {
    const roleWeight: Record<string, number> = {
      authors: 3,
      illustrator: 2,
      editor: 1,
      translator: 1,
      publisher: 1,
    };

    const contributorMap: Record<string, { weight: number; role: string[] }> = {};

    for (const role in roleWeight) {
      const value = json[role];
      if (!value) continue;

      const contributors = Array.isArray(value) ? value : [value];
      for (const wallet of contributors) {
        if (!wallet) continue;

        if (!contributorMap[wallet]) {
          contributorMap[wallet] = { weight: 0, role: [] };
        }

        contributorMap[wallet].weight += roleWeight[role];
        contributorMap[wallet].role.push(role);
      }
    }

    const totalWeight = Object.values(contributorMap).reduce((sum, c) => sum + c.weight, 0);

    let rawShares = Object.entries(contributorMap).map(([wallet, info]) => {
      const exactShare = (info.weight / totalWeight) * 100;
      return {
        wallet,
        roles: info.role,
        exactShare,
        floored: Math.floor(exactShare),
        remainder: exactShare % 1,
      };
    });

    let currentTotal = rawShares.reduce((sum, r) => sum + r.floored, 0);
    let toDistribute = 100 - currentTotal;
    rawShares.sort((a, b) => b.remainder - a.remainder);

    for (let i = 0; i < rawShares.length && toDistribute > 0; i++) {
      rawShares[i].floored += 1;
      toDistribute--;
    }

    rawShares.sort((a, b) => {
      const aIsPublisher = a.roles.includes('publisher') ? -1 : 0;
      const bIsPublisher = b.roles.includes('publisher') ? -1 : 0;
      return aIsPublisher - bIsPublisher;
    });

    return rawShares.map((entry) => ({
      address: new PublicKey(entry.wallet),
      share: entry.floored,
    }));
  };


  const mintNft = async (wallet: any, uploadedJsonUrl: string, pda: any) => {
    try {
      if (!wallet || !wallet.publicKey) {
        console.error("❌ Wallet not connected!");
        return;
      }

      const response = await fetch(uploadedJsonUrl);
      const uploadedJson = await response.json();

      const creators = buildRoyaltyCreators(uploadedJson);
      // ^--- NOTE: Use Metaplex hydra fanout here (precise royalty system)
      console.log("✅ Parsed Creators:", creators);

      const metaplex = Metaplex.make(connection).use(walletAdapterIdentity(wallet));

      await distributeRewards(connection, wallet, creators, 20); // 20 SOL Fixed Price
      const uri = "https://arweave.net/eR4wgSnWusIG-xF2BZzsiOwVehQsvfCT8VAUC4NHQ5Y"; // test URI

      const { nft } = await metaplex.nfts().create({
        uri,
        name: uploadedJson?.title || "The Digital Archive - Book #1",
        sellerFeeBasisPoints: 600,
        creators, //  Same creators list (royalty metadata)!
        collection: new PublicKey(pda), // This is a Hack! needs to be added to uri in the future, we need to ensure collection mechanism is working properly in the future.
        maxSupply: null,
      });

      console.log("✅ NFT minted!");
      console.log(`🧾 NFT Mint Address: ${nft.address.toBase58()}`);
      console.log(`🌐 View on Solana Explorer: https://explorer.solana.com/address/${nft.address.toBase58()}?cluster=devnet`);

    } catch (error) {
      console.error("❌ Error minting NFT:", error);
    }
  };


  return (
    <div className="app-container">
      <h1 className="app-title">The Digital Archive</h1>
      <p className="wallet-info">
        Dev Info (TMP)<br />
        UserW   - Wallet     | Phantom      | CqDhZbsAs41kWYA5wbJ8oMZ5tjhiujfqkdHafGmpp2Cu<br />
        Genesis - Account 0  | Faucet/Admin | HnbV3fxBUZUf3qNKKqucaSqzQ7aBHmjVARU9KrA1cCjL<br />
        Alice   - Account 1  | Author       | 9FS7Y2cq7Bn4YMMV7qUXbX3LbyZZ7zgBYXGiT8nVauSd<br />
        Bob     - Account 2  | Illustrator  | 91frMmiwteBu7Ljfy3vp6bxKSx93EcqpNmpGUT4f5bB6<br />
        Charlie - Account 3  | Editor       | DLCVPSdZH15tVcX5fJNtgBAkoZfZEr7yf7GxZ6VFFi5M<br />
        David   - Account 4  | Translator   | 8dRtfZGRm91XfdfUUyY6g6hW5pgYPdhRXVbEHVwuDvGa<br />
        Ellen   - Account 5  | Jolly        | AHFujZP3Lmh8dffzhA2bqYkNyfhMM3PQK9Xwm1feUTj6<br />
      </p>

      {isClient && <WalletMultiButton className="btn-accent" />}

      {wallet.publicKey && (
        <p className="wallet-info">
          ✅ Connected: {wallet.publicKey.toBase58().slice(0, 5)}...
          {wallet.publicKey.toBase58().slice(-5)}
        </p>
      )}

      <button
        className="btn-accent mt-8"
        onClick={() => router.push('/bookshelf')}
      >
        📚 Explore The Bookshelf
      </button>

      {wallet.publicKey && (
        <div className="card">
          <p className="balance-display">
            {loading ? "Loading..." : `💰 Balance: ${balance} SOL`}
          </p>
          <button onClick={fetchBalance} className="btn-primary mt-4">
            🔄 Refresh Balance
          </button>
        </div>
      )}

      <div className="mt-6 w-full max-w-2xl">
        <input type="file" accept=".json" onChange={handleFileUpload} className="file-input" />
        <button
          className="btn-warning mt-4 w-full"
          onClick={async () => {
            if (!uploadedJson) {
              console.warn("⚠️ No JSON file uploaded yet!");
              return;
            }

            const pda: any = await initializeStorageAccount();
            if (!pda) {
              console.error("❌ Failed to initialize storage account.");
              return;
            }

            let retries = 10;
            while (retries > 0) {
              try {
                const accountInfo = await connection.getAccountInfo(new PublicKey(pda));
                console.log(`🔍 Checking PDA existence... Retries left: ${retries}`, !!accountInfo);
            
                if (accountInfo) {
                  console.log("✅ PDA is now available on-chain.");
                  break;
                }
              } catch (err) {
                console.error("❌ Error during getAccountInfo:", err);
              }
            
              await new Promise((res) => setTimeout(res, 3000));
              retries--;
            }
            

            if (retries === 0) {
              console.error("❌ PDA account not found after multiple attempts.");
              return;
            }

            await storeDataInChunks(UNSAFE_KEY, uploadedJson, pda);
            const metadataJson = {
              ...uploadedJson,
              properties: {
                ...uploadedJson.properties,
                creators: [
                  {
                    address: wallet.publicKey?.toBase58(),
                    share: 100,
                  },
                ],
              },
            };

            const blob = new Blob([JSON.stringify(metadataJson)], { type: "application/json" });
            const url = URL.createObjectURL(blob);
            await mintNft(wallet, url, pda);
          }}
        >
          🚀 Mint Book NFT!
        </button>
      </div>

      <div className="mt-6 w-full">
        <h2 className="text-xl font-semibold">🧾 Retrive Stored Data from Address</h2>

        <input
          type="text"
          className="input-box"
          placeholder="Enter PDA Address"
          value={pdaAddress || ""}
          onChange={(e) => setPdaAddress(e.target.value)}
        />
        <button
          onClick={() => retrieveStoredData(UNSAFE_KEY)}
          className="btn-primary mt-4 w-full"
        >
          📥 Retrieve Stored Data
        </button>
      </div>

      {retrievedContent && (
        <div className="retrieved-content">
          <h2 className="retrieved-content-title">📖 Retrieved Content</h2>
          <pre className="retrieved-content-body">{JSON.stringify(retrievedContent, null, 2)}</pre>
        </div>
      )}
    </div>
  );
}
