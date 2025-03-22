"use client";

import { useState, useEffect } from "react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { useWallet } from "@solana/wallet-adapter-react";
import { Connection, PublicKey } from "@solana/web3.js";
import { useProvider } from "./utils";
import { Program, Idl } from "@project-serum/anchor";
import { Transaction, Message, VersionedTransaction } from "@solana/web3.js";

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

  const connection = new Connection("http://127.0.0.1:8899");
  const PROGRAM_ID = "8Besjdk7LVmnJfuCKAaM2sfAubbggvhgT597XFH8AXbj";

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

  const initializeStorageAccount = async () => {
    if (!anchorBridge || !wallet.signTransaction || !connection) {
      console.warn("⚠️ AnchorBridge instance, wallet signer, or connection not available.");
      return;
    }

    try {
      const totalSize = 900;
      const totalChunks = 10;

      if (!wallet.publicKey) {
        console.error("❌ Wallet is not connected!");
        return;
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

      const transactionId = await connection.sendRawTransaction(signedTransaction.serialize(), { skipPreflight: false, preflightCommitment: "confirmed", });
      console.log("✅ Transaction ID:", transactionId);

      setTxId(transactionId);
      setInitResponse(`Success: ${transactionId}`);

    } catch (error) {
      console.error("❌ Error initializing storage account:", error);
      setInitResponse("Error initializing storage account.");
    }
  };

  const storeDataInChunks = async (key: any, jsonData: any) => {
    if (!anchorBridge || !wallet.signAllTransactions || !connection || !pdaAddress) {
      console.warn("⚠️ Storage account not initialized or wallet unavailable.");
      return;
    }

    try {
      if (!jsonData) {
        console.warn("⚠️ No content to store.");
        return;
      }

      console.log("📖 Preparing JSON data for encryption...");

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
      const txsBase64 = await anchorBridge.store_data_in_chunks(pdaAddress, encrypted_data, 900);
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
  }, [wallet, provider]);

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

      // Remove trailing 0x00 bytes from fetch
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
  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gradient-to-r from-purple-900 to-black text-white p-8">
      <h1 className="text-5xl font-extrabold mb-6 text-transparent bg-clip-text bg-gradient-to-r from-green-400 to-blue-500">
        The Digital Archive
      </h1>

      {/* Wallet Connect Button */}
      <div>
        {isClient && (
          <WalletMultiButton className="!bg-green-600 hover:!bg-green-700 text-white text-lg font-semibold px-6 py-3 rounded-lg shadow-lg transition-all" />
        )}
      </div>

      {/* Display Wallet Address */}
      {wallet.publicKey && (
        <p className="mt-4 text-lg font-medium bg-gray-800 px-4 py-2 rounded-lg shadow-md">
          ✅ Connected: {wallet.publicKey.toBase58().slice(0, 5)}...
          {wallet.publicKey.toBase58().slice(-5)}
        </p>
      )}

      {/* Display Balance */}
      {wallet.publicKey && (
        <div className="mt-6 text-xl font-semibold flex flex-col items-center">
          <p className="bg-gray-900 px-6 py-3 rounded-lg shadow-md">
            {loading ? "Loading..." : `💰 Balance: ${balance} SOL`}
          </p>

          {/* Refresh Balance Button */}
          <button
            onClick={fetchBalance}
            className="mt-4 bg-blue-500 hover:bg-blue-600 text-white px-5 py-2 rounded-lg text-lg font-semibold shadow-md transition-all"
          >
            🔄 Refresh Balance
          </button>
        </div>
      )}

      {/* Button to initialize storage account */}
      <button
        onClick={initializeStorageAccount}
        className="bg-yellow-500 hover:bg-yellow-600 text-black font-bold px-6 py-3 rounded-lg shadow-lg transition-all"
      >
        🚀 Initialize Storage Account
      </button>
      {/* Display PDA Info After Transaction */}

      {pdaAddress && (
        <div className="mt-6 text-lg font-medium bg-gray-800 px-6 py-3 rounded-lg shadow-md">
          <p>📌 <b>PDA Address:</b> {pdaAddress}</p>
          {txId && <p>🔗 <b>Transaction ID:</b> <a target="_blank" className="text-blue-400 underline">{txId}</a></p>}
        </div>
      )}


      <div className="mt-6 w-full max-w-2xl">
        {/* File Upload Input */}
        <input
          type="file"
          accept=".json"
          onChange={handleFileUpload}
          className="w-full bg-gray-800 text-white p-4 rounded-lg shadow-md cursor-pointer"
        />

        <button
          onClick={() => {
            if (!uploadedJson) {
              console.warn("⚠️ No JSON file uploaded yet!");
              return;
            }
            storeDataInChunks("book1234567890123456789012345678", uploadedJson);
          }}
          className="mt-4 bg-green-500 hover:bg-green-600 text-white font-bold px-6 py-3 rounded-lg shadow-lg transition-all w-full"
        >
          📩 Submit Book Content
        </button>

      </div>


      <div className="mt-6 w-full">
        <input
          type="text"
          className="w-full bg-gray-800 text-white p-4 rounded-lg shadow-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="Enter PDA Address"
          value={pdaAddress || ""}
          onChange={(e) => setPdaAddress(e.target.value)}
        />
        <button
          onClick={() => retrieveStoredData("book1234567890123456789012345678")}
          className="mt-4 bg-blue-500 hover:bg-blue-600 text-white font-bold px-6 py-3 rounded-lg shadow-lg transition-all w-full"
        >
          📥 Retrieve Stored Data
        </button>
      </div>

      {retrievedContent && (
        <div className="w-1/2 ml-8 bg-gray-800 p-6 rounded-lg shadow-lg">
          <h2 className="text-xl font-semibold mb-4">📖 Retrieved Content</h2>
          <p className="whitespace-pre-wrap text-gray-300">{retrievedContent}</p>
        </div>
      )}

    </div>
  );
}
