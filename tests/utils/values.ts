import { Program, BN } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Connection, PublicKey, Keypair } from "@solana/web3.js";

import {
    accountExist,
    getConfigAddress,
    getAuthAddress,
    getBlinkAddress,
    getBlinkVaultAddress,
    getSubmitAddress,
 } from "./index";

 export interface TestValues {
    configId:  number;
    config: PublicKey;
    auth: PublicKey;
    tokenMint: PublicKey;
    creatorToken: PublicKey;
    blinkState: PublicKey;
    tokenVault: PublicKey;
    submitState: PublicKey;
    userToken: PublicKey;
}


export function createValues(configId:number, programId: PublicKey): TestValues { 
    const [config, _] = getConfigAddress(configId,programId);
    const [auth, __] = getAuthAddress(programId);

    const tokenMint    = new PublicKey('W8LRujy76DASXHev9VUWdbAUyBZnXmS5MXHKNScPmwW');
    const creatorToken = new PublicKey('Fhi3KpHtC3PGa1V2edV6Ks5N68fX7A2AjP2QDgzutzRG');
    const user         = new PublicKey('FDdjfxEvFjQhgnWbYQeo4GoHb7Kd4RcXVRbqpN4kBc9M');

    const [blinkState, ___] = getBlinkAddress(config, tokenMint, programId);
    const [tokenVault, ____] = getBlinkVaultAddress(blinkState, tokenMint, programId);
    const [submitState, _____] = getSubmitAddress(blinkState, user, programId);
    const userToken = creatorToken;


    return {
      configId,
      config,  
      auth,
      tokenMint,
      creatorToken,
      blinkState,
      tokenVault,
      submitState,
      userToken,
    };
}