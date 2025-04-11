import { Connection, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";

export const distributeRewards = async (
  connection: Connection,
  wallet: any,
  creators: { address: PublicKey; share: number }[],
  totalPriceSol: number
) => {
  if (!wallet || !wallet.publicKey) {
    throw new Error("Wallet not connected");
  }

  const buyer = wallet.publicKey;
  const totalPriceLamports = totalPriceSol * 1_000_000_000; // 1 SOL = 10^9 Lamports

  const buyerBalance = await connection.getBalance(buyer);
  if (buyerBalance < totalPriceLamports) {
    throw new Error(`Insufficient balance to purchase. Price: ${totalPriceSol} SOL`);
  }

  const instructions = creators.map(({ address, share }) =>
    SystemProgram.transfer({
      fromPubkey: buyer,
      toPubkey: address,
      lamports: Math.floor((totalPriceLamports * share) / 100),
    })
  );

  const tx = new Transaction().add(...instructions);

  const sig = await wallet.sendTransaction(tx, connection);
  await connection.confirmTransaction(sig, "confirmed");

  console.log("✅ Rewards distributed successfully");
};
