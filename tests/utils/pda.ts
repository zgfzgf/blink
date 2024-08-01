import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

export const CONFIG_SEED = Buffer.from(
    anchor.utils.bytes.utf8.encode("config_seed")
);
export const AUTH_SEED = Buffer.from(
    anchor.utils.bytes.utf8.encode("auth_seed")
);
export const BLINK_SEED= Buffer.from(
    anchor.utils.bytes.utf8.encode("blink_seed")
);
export const BLINK_VAULT_SEED= Buffer.from(
    anchor.utils.bytes.utf8.encode("blink_vault_seed")
);

export const SUBMIT_SEED= Buffer.from(
    anchor.utils.bytes.utf8.encode("submit_seed")
);

export function u16ToBytes(num: number) {
  const arr = new ArrayBuffer(2);
  const view = new DataView(arr);
  view.setUint16(0, num, false);
  return new Uint8Array(arr);
} 

export function getConfigAddress(
    index: number,
    programId: PublicKey
  ): [PublicKey, number] {
    const [address, bump] = PublicKey.findProgramAddressSync(
      [CONFIG_SEED, Buffer.from([index])],
      programId
    );
    return [address, bump];
  }

  export function getConfigAddress1(
    owner: PublicKey,
    programId: PublicKey
  ): [PublicKey, number] {
    const [address, bump] = PublicKey.findProgramAddressSync(
      [CONFIG_SEED, owner.toBuffer()],
      programId
    );
    return [address, bump];
  }


  export function getAuthAddress(
    programId: PublicKey
  ): [PublicKey, number] {
    const [address, bump] = PublicKey.findProgramAddressSync(
      [AUTH_SEED],
      programId
    );
    return [address, bump];
  }
  
  export function getBlinkAddress(
    config: PublicKey,
    tokenMint: PublicKey,
    programId: PublicKey
  ): [PublicKey, number] {
    const [address, bump] = PublicKey.findProgramAddressSync(
      [
        BLINK_SEED,
        config.toBuffer(),
        tokenMint.toBuffer(),
      ],
      programId
    );
    return [address, bump];
  }

  export function getBlinkVaultAddress(
    blink: PublicKey,
    tokenMint: PublicKey,
    programId: PublicKey
  ): [PublicKey, number] {
    const [address, bump] = PublicKey.findProgramAddressSync(
      [BLINK_VAULT_SEED, blink.toBuffer(), tokenMint.toBuffer()],
      programId
    );
    return [address, bump];
  }

  export function getSubmitAddress(
    blink: PublicKey,
    user: PublicKey,
    programId: PublicKey
  ): [PublicKey, number] {
    const [address, bump] = PublicKey.findProgramAddressSync(
      [SUBMIT_SEED, blink.toBuffer(), user.toBuffer()],
      programId
    );
    return [address, bump];
  }

