import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Blink } from "../target/types/blink";
import { PublicKey, Keypair } from "@solana/web3.js";
import {
  getAuthAddress,
  getTimeAddress,
  getConfigAddress,
  getBlinkAddress,
  getSubmitAddress,
  getSplTokenAddress,
  getKeypair,
} from "./utils";
import { BN } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("blink start", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Blink as Program<Blink>;
  const provider = program.provider as anchor.AnchorProvider;
  const payer = provider.wallet as anchor.Wallet;

  const index = 4016;
  const answer = 3;
  const amount = new BN(10 ** 9);
  const openTime = new BN(new Date().getTime() / 1000 + 60);
  const period = new BN(600);

  const tokenMint = new PublicKey(
    "W8LRujy76DASXHev9VUWdbAUyBZnXmS5MXHKNScPmwW"
  );

  let creator, user, owner: Keypair;
  let auth, config, blink, submit, vault, creatorToken, userToken: PublicKey;
  let timeConfig: PublicKey;

  const confirm = async (signature: string): Promise<string> => {
    const block = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      signature,
      ...block,
    });
    return signature;
  };

  function sleep(mm: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, mm * 1000));
  }

  beforeEach(() => {
    let _;
    [auth, _] = getAuthAddress(program.programId);
    [timeConfig, _] = getTimeAddress(program.programId);
    [config, _] = getConfigAddress(index, program.programId);

    [blink, _] = getBlinkAddress(index, program.programId);
    vault = getSplTokenAddress(tokenMint, auth, TOKEN_PROGRAM_ID);

    owner = getKeypair("./owner.json");
    creator = getKeypair("./creator.json");
    user = getKeypair("./user.json");

    creatorToken = getSplTokenAddress(
      tokenMint,
      creator.publicKey,
      TOKEN_PROGRAM_ID
    );
    userToken = getSplTokenAddress(tokenMint, user.publicKey, TOKEN_PROGRAM_ID);
    [submit, _] = getSubmitAddress(index, user.publicKey, program.programId);
    /*
    console.log(`TOKEN_PROGRAM      : ${TOKEN_PROGRAM_ID}`);
    console.log(`program.programId  : ${program.programId}`);
    console.log(`Creator Address    : ${creator.publicKey}`);
    console.log(`CreatorToken       : ${creatorToken}`);
    console.log(``);
    console.log(`Payer Address      : ${payer.publicKey}`);
    console.log(`Program Address    : ${program.programId}`);
    console.log(`Auth Address       : ${auth}`);
    console.log(`Blink Address      : ${blink}`);
    console.log(`Submit Address     : ${submit}`);
    console.log(`CreatorToken       : ${creatorToken}`);
    console.log(`Vault Address      : ${vault}`);
    console.log(`Config Address   ->: ${config}`);
    */
  });
  const configInfo = async () => {
    console.log(`config info`);
    const configInfo = await program.account.blinkConfig.fetch(config);
    console.log(`index     : ${configInfo.index}`);
    console.log(`pic       : ${configInfo.pic}`);
    console.log(`content   : ${configInfo.content}`);
    console.log(`option1   : ${configInfo.option1}`);
    console.log(`option2   : ${configInfo.option2}`);
    console.log(`option3   : ${configInfo.option3}`);
    console.log(`option4   : ${configInfo.option4}`);
  };
  const blinkInfo = async () => {
    console.log(`blink info`);
    const blinkInfo = await program.account.blinkState.fetch(blink);
    console.log(`index      : ${blinkInfo.index}`);
    console.log(`closed     : ${blinkInfo.closed}`);
    console.log(`amount     : ${blinkInfo.amount}`);
    console.log(`answer     : ${blinkInfo.answer}`);
    console.log(`reward     : ${blinkInfo.reward}`);
    console.log(`right1     : ${blinkInfo.right1}`);
    console.log(`right2     : ${blinkInfo.right2}`);
    console.log(`right3     : ${blinkInfo.right3}`);
    console.log(`right4     : ${blinkInfo.right4}`);
  };
  const submitInfo = async () => {
    console.log(`submit info`);
    const submitInfo = await program.account.submitState.fetch(submit);
    console.log(`index     : ${submitInfo.index}`);
    console.log(`answer    : ${submitInfo.answer}`);
    console.log(`claim     : ${submitInfo.claim}`);
    console.log(`user      : ${submitInfo.user}`);
  };

  it("update func", async () => {
    await program.methods
      .updateTime(openTime, period)
      .accounts({
        owner: owner.publicKey,
        timeConfig: timeConfig,
      })
      .signers([owner])
      .rpc()
      .then(confirm);
  });

  it("Errot Init Test", async () => {
    try {
      await program.methods
        .initialize(index, amount, "pic", "content", "op1", "op2", "op3", "op4")
        .accounts({
          creator: creator.publicKey,
          authority: auth,
          timeConfig: timeConfig,
          blinkConfig: config,
          blinkState: blink,
          tokenMint: tokenMint,
          creatorToken: creatorToken,
          vault: vault,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([user])
        .rpc()
        .then(confirm);
    } catch (error) {
      console.log("\nError user======");
      console.log(error);
    }
  });

  it("initialize func", async () => {
    await program.methods
      .initialize(index, amount, "pic", "content", "op1", "op2", "op3", "op4")
      .accounts({
        creator: creator.publicKey,
        authority: auth,
        timeConfig: timeConfig,
        blinkConfig: config,
        blinkState: blink,
        tokenMint: tokenMint,
        creatorToken: creatorToken,
        vault: vault,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([creator])
      .rpc()
      .then(confirm);
  });

  it("Read config and blink data", async () => {
    await configInfo();
    await blinkInfo();
  });

  it("Errot Init Test", async () => {
    try {
      await program.methods
        .initialize(index, amount, "pic", "content", "op1", "op2", "op3", "op4")
        .accounts({
          creator: creator.publicKey,
          authority: auth,
          timeConfig: timeConfig,
          blinkConfig: config,
          blinkState: blink,
          tokenMint: tokenMint,
          creatorToken: creatorToken,
          vault: vault,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([creator])
        .rpc()
        .then(confirm);
    } catch (error) {
      console.log("\nError index======");
      console.log(error);
    }
  });

  it("Errot Submit Test", async () => {
    try {
      await program.methods
        .submit(index, answer)
        .accounts({
          user: user.publicKey,
          submitState: submit,
          blinkState: blink,
        })
        .signers([creator])
        .rpc()
        .then(confirm);
    } catch (error) {
      console.log("\nError submit======");
      console.log(error);
    }
  });

  it("Errot submit func", async () => {
    try {
      await program.methods
        .submit(index, answer)
        .accounts({
          user: user.publicKey,
          submitState: submit,
          blinkState: blink,
        })
        .signers([user])
        .rpc()
        .then(confirm);
    } catch (error) {
      console.log("\nError submit======");
      console.log(error);
    }
  });

  it("submit func", async () => {
    await sleep(60);
    await program.methods
      .submit(index, answer)
      .accounts({
        user: user.publicKey,
        submitState: submit,
        blinkState: blink,
      })
      .signers([user])
      .rpc()
      .then(confirm);
  });

  it("Read submit data", async () => {
    await submitInfo();
  });

  it("Errot Submit Test", async () => {
    try {
      await program.methods
        .submit(index, answer)
        .accounts({
          user: user.publicKey,
          submitState: submit,
          blinkState: blink,
        })
        .signers([user])
        .rpc()
        .then(confirm);
    } catch (error) {
      console.log("\nError submit======");
      console.log(error);
    }
  });

  it("Errot Close Test", async () => {
    try {
      await program.methods
        .close(index, answer)
        .accounts({
          owner: user.publicKey,
          blinkState: blink,
        })
        .signers([user])
        .rpc()
        .then(confirm);
    } catch (error) {
      console.log("\nError close======");
      console.log(error);
    }
  });

  it("Errot close func", async () => {
    try {
      await program.methods
        .close(index, answer)
        .accounts({
          owner: creator.publicKey,
          blinkState: blink,
        })
        .signers([creator])
        .rpc()
        .then(confirm);
    } catch (error) {
      console.log("\nError close======");
      console.log(error);
    }
  });

  it("close func", async () => {
    await sleep(600);
    await program.methods
      .close(index, answer)
      .accounts({
        payer: creator.publicKey,
        blinkState: blink,
      })
      .signers([creator])
      .rpc()
      .then(confirm);
  });

  it("Read blink  data", async () => {
    await blinkInfo();
  });

  it("Errot Close Test", async () => {
    try {
      await program.methods
        .close(index, answer)
        .accounts({
          payer: creator.publicKey,
          blinkState: blink,
        })
        .signers([creator])
        .rpc()
        .then(confirm);
    } catch (error) {
      console.log("\nError close======");
      console.log(error);
    }
  });

  it("Errot Claim Test", async () => {
    try {
      await program.methods
        .claim(index)
        .accounts({
          user: user.publicKey,
          submitState: submit,
          blink_state: blink,
          authority: auth,
          user_account: userToken,
          vault: vault,
          tokenMint: tokenMint,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([creator])
        .rpc()
        .then(confirm);
    } catch (error) {
      console.log("\nError claim======");
      console.log(error);
    }
  });

  it("claim func", async () => {
    await program.methods
      .claim(index)
      .accounts({
        user: user.publicKey,
        submitState: submit,
        blink_state: blink,
        authority: auth,
        user_account: userToken,
        vault: vault,
        tokenMint: tokenMint,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc()
      .then(confirm);
  });

  it("Read binkState and sumbit data", async () => {
    await blinkInfo();
    await submitInfo();
  });
  it("Errot Claim Test", async () => {
    try {
      await program.methods
        .claim(index)
        .accounts({
          user: user.publicKey,
          submitState: submit,
          blink_state: blink,
          authority: auth,
          user_account: userToken,
          vault: vault,
          tokenMint: tokenMint,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([user])
        .rpc()
        .then(confirm);
    } catch (error) {
      console.log("\nError claim======");
      console.log(error);
    }
  });
});
