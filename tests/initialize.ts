import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Blink } from "../target/types/blink";
import { type TestValues, createValues } from './utils';
import { BN } from 'bn.js';

describe("initialize start", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());
  
    const program = anchor.workspace.Blink as Program<Blink>;
    const provider = program.provider as anchor.AnchorProvider;
    const payer = provider.wallet as anchor.Wallet;
  
    let values: TestValues;
    let index: number;
    index = 30;
    beforeEach(() => {
        values = createValues(index, program.programId);
    });
  
    it('initialize func', async () => {
      console.log(`Payer Address      : ${payer.publicKey}`);
      console.log(`Program Address    : ${program.programId}`);
      console.log(`Config Address     : ${values.config}`);
      console.log(`Auth Address       : ${values.auth}`);
      console.log(`Blink Address      : ${values.blinkState}`);
      console.log(`Vault Address      : ${values.tokenVault}`);



      const amount = new BN(100);
      const answer = 3;
  
      await program.methods
        .initialize(amount, answer)
        .accounts({
          creator: payer.publicKey,
          blinkConfig: values.config,
          // authority:values.auth,
          blinkState:values.blinkState,
          tokenMint: values.tokenMint,
          creatorToken: values.creatorToken,
          //tokenVault: values.tokenVault,
        })
        .rpc();
    });
  
    it("Read the new account's data", async () => {
      const blinkInfo = await program.account.blinkState.fetch(values.blinkState);
      console.log(`reward     : ${blinkInfo.reward}`);
    });
  
    
  
    
  
  });