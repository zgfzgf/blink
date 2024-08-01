import { createMint, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import { Connection, Keypair,  Signer } from '@solana/web3.js';

export const mintingTokens = async ({
    connection,
    creator,
    holder = creator,
    mintKeypair,
    mintedAmount = 100,
    decimals = 6,
  }: {
    connection: Connection;
    creator: Signer;
    holder?: Signer;
    mintKeypair: Keypair;
    mintedAmount?: number;
    decimals?: number;
  }) => {
    // Mint tokens
    await createMint(connection, creator, creator.publicKey, creator.publicKey, decimals, mintKeypair);
    await getOrCreateAssociatedTokenAccount(connection, holder, mintKeypair.publicKey, holder.publicKey, true);
    await mintTo(
      connection,
      creator,
      mintKeypair.publicKey,
      getAssociatedTokenAddressSync(mintKeypair.publicKey, holder.publicKey, true),
      creator.publicKey,
      mintedAmount * 10 ** decimals,
    );

  };