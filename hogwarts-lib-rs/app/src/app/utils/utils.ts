import { Connection, PublicKey } from "@solana/web3.js";
  
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
  