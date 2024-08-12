import { Keypair } from "@solana/web3.js";

export function getKeypair(file: string): Keypair {
  return Keypair.fromSecretKey(
    Uint8Array.from(
      JSON.parse(
        require("fs").readFileSync(file, {
          encoding: "utf-8",
        })
      )
    )
  );
}
