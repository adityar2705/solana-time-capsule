import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SystemProgram } from "@coral-xyz/anchor";
import { assert } from "chai";
import { SolanaTimeCapsule } from "../target/types/solana_time_capsule";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("solana-time-capsule", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  //get an instance of the time capsule smart contract
  const program = anchor.workspace.SolanaTimeCapsule as Program<SolanaTimeCapsule>;
  const baseAccount = anchor.web3.Keypair.generate();

  it("initialized the time capsule", async () => {
    //call the initialize function
    const tx = await program.methods
    .initialize("The first data in my time capsule")
    .accounts({
      baseAccount:baseAccount.publicKey,
      user:provider.wallet.publicKey,
    })
    .signers([baseAccount])
    .rpc();

    //checking whether the data has been set or not
    const account = program.account.baseAccount.fetch(baseAccount.publicKey);
    assert.ok((await account).data == "The first data in my time capsule");
  });

  it("updated the time capsule", async () => {
    //calling the update function
    const tx = await program.methods
    .update("The second data in my time capsule")
    .accounts({
      baseAccount:baseAccount.publicKey,
      user: provider.wallet.publicKey,
    })
    .rpc();

    //getting the account before updating
    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log(`✅ Transaction was successful!`);
    assert.ok(account.data == "The second data in my time capsule");
  }); 
});
