import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ComputeUnit } from "../target/types/compute_unit";

describe("compute_unit", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.computeUnit as Program<ComputeUnit>;

  const defaultKeyPair = new anchor.web3.PublicKey(
    "EbBpJhNj6XNRhZFVokobLpce7brHcr9X5pE9EHpHhoHH"
  );

  it("Is initialized!", async () => {
    // log the keypair's initial balance
    let bal_before = await program.provider.connection.getBalance(
      defaultKeyPair
    );
    console.log("before:", bal_before);

    // call the initialize function of our program
    const tx = await program.methods.initialize().rpc();

    // log the keypair's balance after
    let bal_after = await program.provider.connection.getBalance(
      defaultKeyPair
    );
    console.log("after:", bal_after);

    // log the difference
    console.log(
      "diff:",
      BigInt(bal_before.toString()) - BigInt(bal_after.toString())
    );
  });
});
