import * as anchor from "@coral-xyz/anchor";
import { BN } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

import { getAssociatedTokenAddressSync } from "@solana/spl-token";

export const AUTH_SEED = Buffer.from(
  anchor.utils.bytes.utf8.encode("auth_seed")
);

export const TIME_SEED = Buffer.from(
  anchor.utils.bytes.utf8.encode("time_seed")
);

export const CONFIG_SEED = Buffer.from(
  anchor.utils.bytes.utf8.encode("config_seed")
);

export const BLINK_SEED = Buffer.from(
  anchor.utils.bytes.utf8.encode("blink_seed")
);

export const SUBMIT_SEED = Buffer.from(
  anchor.utils.bytes.utf8.encode("submit_seed")
);

export function numberToBytes(index: number) {
  const num = new BN(index);
  return num.toArrayLike(Buffer, "le", 2);
}

export function getTimeAddress(programId: PublicKey): [PublicKey, number] {
  const [address, bump] = PublicKey.findProgramAddressSync(
    [TIME_SEED],
    programId
  );
  return [address, bump];
}

export function getAuthAddress(programId: PublicKey): [PublicKey, number] {
  const [address, bump] = PublicKey.findProgramAddressSync(
    [AUTH_SEED],
    programId
  );
  return [address, bump];
}

export function getConfigAddress(
  index: number,
  programId: PublicKey
): [PublicKey, number] {
  const [address, bump] = PublicKey.findProgramAddressSync(
    [CONFIG_SEED, numberToBytes(index)],
    programId
  );
  return [address, bump];
}

export function getBlinkAddress(
  index: number,
  programId: PublicKey
): [PublicKey, number] {
  const [address, bump] = PublicKey.findProgramAddressSync(
    [BLINK_SEED, numberToBytes(index)],
    programId
  );
  return [address, bump];
}

export function getSplTokenAddress(
  tokenMint: PublicKey,
  authority: PublicKey,
  programId: PublicKey
): PublicKey {
  return getAssociatedTokenAddressSync(tokenMint, authority, true, programId);
}

export function getSubmitAddress(
  index: number,
  user: PublicKey,
  programId: PublicKey
): [PublicKey, number] {
  const [address, bump] = PublicKey.findProgramAddressSync(
    [SUBMIT_SEED, numberToBytes(index), user.toBuffer()],
    programId
  );
  return [address, bump];
}
