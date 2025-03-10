import { AnchorProvider, Program, Idl } from "@project-serum/anchor";
import { Connection, PublicKey } from "@solana/web3.js";
import { Wallet, useWallet } from "@solana/wallet-adapter-react";
import bookStorageIdl from "/home/rzanei/dev/the-archive/hogwarts-lib-rs/target/idl/book_storage.json";

const SOLANA_RPC_URL = "http://127.0.0.1:8899";

export const useProvider = () => {
  const { publicKey, signTransaction, signAllTransactions } = useWallet();

  if (!publicKey || !signTransaction) return null;

  const connection = new Connection(SOLANA_RPC_URL, "processed");

  return new AnchorProvider(
    connection,
    {
      publicKey,
      signTransaction,
      signAllTransactions: signAllTransactions || (async (txs) => Promise.all(txs.map(signTransaction))),
    },
    { preflightCommitment: "processed" }
  );
};

export const useProgram = () => {
  const provider = useProvider();
  if (!provider) {
    console.error("❌ Provider not available! Program cannot be created.");
    return null;
  }

  try {
    const programId = new PublicKey(bookStorageIdl.address);
    const bookStorageIdlTyped: Idl = bookStorageIdl as unknown as Idl;
    const program = new Program(bookStorageIdlTyped, programId, provider);
    console.log("✅ Successfully created program:", program);
    return program;
  } catch (error) {
    console.error("❌ Error initializing program:", error);
    return null;
  }
};
