import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SizeLimit } from "../target/types/size_limit";

describe("size_limit", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SizeLimit as Program<SizeLimit>;

  it("Initialize and set value", async () => {

    // seeds has two values
    const seeds = [];
    let valueAccount = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    )[0];

    console.log(valueAccount)

    // functions now take two keys
    await program.methods.initialize().accounts({myStorage: valueAccount}).rpc();

    await program.methods.increaseAccountSize().accounts({myStorage: valueAccount}).rpc();
  });
});
