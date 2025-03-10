"use client";
import { useState } from "react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { useWallet } from "@solana/wallet-adapter-react";

export default function Home() {
  const { publicKey } = useWallet();
  const [message, setMessage] = useState("Connect your wallet to start");

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 text-white p-8">
      <h1 className="text-4xl font-bold mb-4">Solana dApp</h1>
      <WalletMultiButton />
      
      {publicKey && (
        <p className="mt-4 text-lg">
          ✅ Connected: {publicKey.toBase58()}
        </p>
      )}
      
      <button
        className="mt-6 bg-blue-500 hover:bg-blue-600 px-6 py-2 rounded-lg"
        onClick={() => setMessage("You clicked the button!")}
      >
        Click Me
      </button>
      
      <p className="mt-4">{message}</p>
    </div>
  );
}

