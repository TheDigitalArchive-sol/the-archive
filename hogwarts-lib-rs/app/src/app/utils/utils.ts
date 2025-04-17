import { Connection, PublicKey } from "@solana/web3.js";
import { useEffect, useState } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
  
export const retrieveStoredData = async (
    connection: Connection,
    pdaAddress: any,
    anchorBridge: any,
    key: string
  ): Promise<string | null> => {
    if (!connection || !pdaAddress || !anchorBridge) {
      console.warn("⚠️ Connection, Storage Account PDA, or AnchorBridge not available.");
      return null;
    }
  
    try {
      console.log("📥 Retrieving stored book data...");
      const accountInfo = await connection.getAccountInfo(new PublicKey(pdaAddress));
      if (!accountInfo || !accountInfo.data) {
        console.error("❌ No data found in storage account!");
        return null;
      }
  
      let storedBytes = accountInfo.data.slice(20);
      while (storedBytes.length > 0 && storedBytes[storedBytes.length - 1] === 0) {
        storedBytes = storedBytes.slice(0, -1);
      }
  
      if (!storedBytes || storedBytes.length === 0) {
        console.error("❌ No valid data found in storage account!");
        return null;
      }
  
      let storedText;
      try {
        storedText = await anchorBridge.light_msg_decryption(key, storedBytes);
      } catch (error) {
        console.error("❌ Decryption failed:", error);
        return null;
      }
  
      console.log("📖 Stored Book Content:", storedText);
      return storedText;
    } catch (error) {
      console.error("❌ Error retrieving stored data:", error);
      return null;
    }
  };
  
  export function useAnchorBridge() {
    const [anchorBridge, setAnchorBridge] = useState<any | null>(null);
    const [wasm, setWasm] = useState<any | null>(null);
    const wallet = useWallet();
  
    const PROGRAM_ID = process.env.NEXT_PUBLIC_PROGRAM_ID!;
  
    useEffect(() => {
      if (!PROGRAM_ID) {
        console.error("❌ Missing PROGRAM_ID!");
        return;
      }
  
      import("/home/rzanei/dev/the-archive/powerwand/pkg/powerwand.js")
        .then(async (module) => {
          await module.default();
          setWasm(module);
        })
        .catch((error) => console.error("❌ Error loading WASM module:", error));
    }, []);
  
    useEffect(() => {
      if (!wasm || !wallet.connected || !wallet.publicKey) return;
  
      try {
        console.log("✅ Wallet connected, initializing AnchorBridge...");
        const bridge = new wasm.AnchorBridge(wallet.publicKey.toBase58(), PROGRAM_ID);
        setAnchorBridge(bridge);
      } catch (error) {
        console.error("❌ Error creating AnchorBridge:", error);
      }
    }, [wasm, wallet.connected, wallet.publicKey]);
  
    return { anchorBridge, wasm };
  }
  