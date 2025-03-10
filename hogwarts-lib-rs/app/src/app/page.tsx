// This file defines a specific page in your application. It typically contains the logic and UI related to that page.

// What it should contain:
// The main structure and content for a single page. This could be a dashboard, home page, or specific page for interacting with your Solana smart contracts.
// Calls to APIs or smart contracts (via the frontend Solana client or Anchor's frontend methods).
// Event handlers for user interactions (e.g., button clicks).

"use client";

import { useState, useEffect } from "react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { useWallet } from "@solana/wallet-adapter-react";
import { Connection, clusterApiUrl, PublicKey } from "@solana/web3.js";

export default function Home() {
  const { publicKey } = useWallet(); // Get connected wallet
  const [balance, setBalance] = useState<number | null>(null);
  const [loading, setLoading] = useState(false);

  // Solana connection
  const connection = new Connection("http://127.0.0.1:8899");

  // Function to fetch balance
  const fetchBalance = async () => {
    if (!publicKey) return;
    setLoading(true);
    try {
      const balance = await connection.getBalance(new PublicKey(publicKey));
      setBalance(balance / 1e9); // Convert lamports to SOL
    } catch (error) {
      console.error("Error fetching balance:", error);
    }
    setLoading(false);
  };

  // Fetch balance on wallet connect
  useEffect(() => {
    if (publicKey) fetchBalance();
  }, [publicKey]);

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gradient-to-r from-purple-900 to-black text-white p-8">
      <h1 className="text-5xl font-extrabold mb-6 text-transparent bg-clip-text bg-gradient-to-r from-green-400 to-blue-500">
        Solana dApp
      </h1>

      {/* Wallet Connect Button */}
      <div className="mb-6">
        <WalletMultiButton className="!bg-green-600 hover:!bg-green-700 text-white text-lg font-semibold px-6 py-3 rounded-lg shadow-lg transition-all" />
      </div>

      {/* Display Wallet Address */}
      {publicKey && (
        <p className="mt-4 text-lg font-medium bg-gray-800 px-4 py-2 rounded-lg shadow-md">
          ✅ Connected: {publicKey.toBase58().slice(0, 5)}...
          {publicKey.toBase58().slice(-5)}
        </p>
      )}

      {/* Display Balance */}
      {publicKey && (
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
