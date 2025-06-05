import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Tryrust } from "../target/types/tryrust";

describe("tryrust", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.tryrust as Program<Tryrust>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize("name", "Bob", "Alice", new anchor.BN(20)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Age checker", async () => {
    // Add your test here.
    const tx = await program.methods.ageChecker(new anchor.BN(35)).rpc();
    console.log("Your transaction signature", tx);
  });
});
