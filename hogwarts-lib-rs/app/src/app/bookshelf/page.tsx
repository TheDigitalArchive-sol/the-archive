'use client';

import React, { useEffect, useState } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { isMetadata, Metaplex, walletAdapterIdentity } from '@metaplex-foundation/js';
import { Connection, PublicKey } from '@solana/web3.js';

type NftDisplay = {
  mintAddress: string;
  title: string;
  image: string;
  uri: string;
};

export default function BookshelfPage() {
  const [nfts, setNfts] = useState<NftDisplay[]>([]);
  const [loading, setLoading] = useState(true);

  const wallet = useWallet();
  const connection = new Connection('http://127.0.0.1:8899');
  const ORG_CREATOR_PUBKEY = process.env.NEXT_PUBLIC_ORG_CREATOR_PUBKEY!;

  useEffect(() => {
    const fetchNFTs = async () => {
      const metaplex = Metaplex.make(connection).use(walletAdapterIdentity(wallet));
      const OCP = new PublicKey(ORG_CREATOR_PUBKEY);
  
      try {
        const positions = [0, 1, 2, 3, 4];
  
        const allResults = await Promise.all(
          positions.map((position) =>
            metaplex.nfts()
              .findAllByCreator({ creator: OCP, position })
              .catch((e) => {
                console.warn(`Position ${position} fetch failed`, e);
                return [];
              })
          )
        );
  
        const metadataList = allResults.flat();
        const loaded: NftDisplay[] = [];
  
        for (const metadata of metadataList) {
          try {
            const nft = await metaplex.nfts().findByMint({
              mintAddress: isMetadata(metadata) ? metadata.mintAddress : metadata.address
              ,
            });
  
            const includesOrg = (nft.creators ?? []).some(
              (c) => c.address.toBase58() === OCP.toBase58()
            );
  
            if (!includesOrg || !nft.json?.image) continue;
  
            loaded.push({
              mintAddress: nft.address.toBase58(),
              title: nft.name,
              image: nft.json.image,
              uri: nft.uri,
            });
  
            await new Promise((r) => setTimeout(r, 100)); // Rate limit!!
          } catch (e) {
            console.warn('⚠️ Failed to load NFT metadata:', e);
          }
        }
  
        setNfts(loaded);
      } catch (e) {
        console.error('❌ Failed to fetch NFTs:', e);
      } finally {
        setLoading(false);
      }
    };
  
    fetchNFTs();
  }, []);
  
  
  return (
    <div className="app-container">
      <h1 className="app-title mb-8">The Digital Archive Bookshelf</h1>

      {loading ? (
        <p className="text-gray-500">Loading your minted books...</p>
      ) : nfts.length === 0 ? (
        <p className="text-gray-500">You haven’t minted any books yet. Try minting one!</p>
      ) : (
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
          {nfts.map((nft) => (
            <div
              key={nft.mintAddress}
              className="bg-black rounded-xl shadow-lg p-4 flex flex-col items-center"
            >
              <img
                src={nft.image}
                alt={nft.title}
                className="w-full h-64 object-cover rounded-lg mb-4"
              />
              <p className="text-lg font-semibold text-white mb-2">{nft.title}</p>
              <p className="text-sm text-gray-400 mb-4">
                🪙 {nft.mintAddress.slice(0, 6)}...{nft.mintAddress.slice(-4)}
              </p>

              <div className="flex flex-col w-full gap-2">
                <a
                  href={`https://explorer.solana.com/address/${nft.mintAddress}?cluster=devnet`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="btn-secondary text-center"
                >
                  📖 View on Explorer
                </a>
                <button
                  onClick={() => console.log(`Mint again ${nft.mintAddress}`)}
                  className="btn-primary"
                >
                  ✨ Buy a Copy
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
