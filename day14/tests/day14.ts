import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day14 } from "../target/types/day14";

describe("Day14", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // generate a signer to call our function
  let myKeypair = anchor.web3.Keypair.generate();

  const program = anchor.workspace.Day14 as Program<Day14>;

  it("Is signed by multiple signers", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({
        signer1: program.provider.publicKey,
        signer2: myKeypair.publicKey,
      })
      .signers([myKeypair])// @note appends the second signer to list of signers.
      .rpc();

    console.log("The signer1: ", program.provider.publicKey.toBase58());
    console.log("The signer2: ", myKeypair.publicKey.toBase58());

    console.log("Transaction hash:", tx);
  });

    it("Is NOT called by the owner", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({
        signer1: myKeypair.publicKey,
        signer2: program.provider.publicKey,
      })
      .signers([myKeypair])
      .rpc();

    console.log("Transaction hash:", tx);
  });
});

