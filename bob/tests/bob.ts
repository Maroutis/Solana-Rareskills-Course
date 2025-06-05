import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Bob } from "../target/types/bob";
import { Alice } from "../target/types/alice";
import { expect } from "chai";

describe("CPI from Alice to Bob", () => {
  const provider = anchor.AnchorProvider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const bobProgram = anchor.workspace.Bob as Program<Bob>;
  const aliceProgram = anchor.workspace.Alice as Program<Alice>;
  const dataAccountKeypair = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await bobProgram.methods
      .initialize()
      .accounts({
        bobDataAccount: dataAccountKeypair.publicKey,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([dataAccountKeypair])// Because you are using a raw Keypair for a non-PDA account, that Keypair must sign the CreateAccount call. That’s why you put .signers([dataAccountKeypair]). In Solana’s create_account instruction, the “new account” must be designated by a Keypair and must sign the transaction—otherwise the runtime won’t know “yes, I really do own that new address.”
      .rpc();
  });

  it("Can add numbers then double!", async () => {
    // Add your test here.
    const tx = await aliceProgram.methods
      .askBobToAddThenDouble(new anchor.BN(4), new anchor.BN(2))
      .accounts({
        bobDataAccount: dataAccountKeypair.publicKey,
        bobProgram: bobProgram.programId,
      })
      .rpc();
  });

  it("Can assert value in Bob's data account equals 4 + 2", async () => {

    const BobAccountValue = (
      await bobProgram.account.bobData.fetch(dataAccountKeypair.publicKey)    ).result.toNumber();
    expect(BobAccountValue).to.equal(6);
  });
});

