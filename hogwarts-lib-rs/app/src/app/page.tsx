"use client";

import { useState, useEffect } from "react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { useWallet } from "@solana/wallet-adapter-react";
import { Connection, PublicKey } from "@solana/web3.js";
import { useProgram, useProvider } from "./utils";
import { Program, Idl } from "@project-serum/anchor";

export default function Home() {
    const [isClient, setIsClient] = useState(false);

    const wallet = useWallet();
    const [balance, setBalance] = useState<number | null>(null);
    const [loading, setLoading] = useState(false);
    const [program, setProgram] = useState<Program<Idl> | null>(null);

    const connection = new Connection("http://127.0.0.1:8899");

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

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gradient-to-r from-purple-900 to-black text-white p-8">
      <h1 className="text-5xl font-extrabold mb-6 text-transparent bg-clip-text bg-gradient-to-r from-green-400 to-blue-500">
        Solana dApp
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
          </div>
  );
}
