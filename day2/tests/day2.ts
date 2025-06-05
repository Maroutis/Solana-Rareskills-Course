import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day2 } from "../target/types/day2";

describe("day2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day2 as Program<Day2>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(new anchor.BN(777), new anchor.BN(888), "hello").rpc();
    console.log("Your transaction signature", tx);
  });

  it("Array test", async () => {
  const tx = await program.methods.array([new anchor.BN(777), new anchor.BN(888)]).rpc();
  console.log("Your transaction signature", tx);
  })

  it("Adds two numbers", async () => {
  const tx = await program.methods.add(new anchor.BN(777), new anchor.BN(888)).rpc();
  console.log("Your transaction signature", tx);
  })

  it("Substract two numbers", async () => {
  const tx = await program.methods.sub(new anchor.BN(888), new anchor.BN(777)).rpc();
  console.log("Your transaction signature", tx);
  })

  it("Multiply two numbers", async () => {
  const tx = await program.methods.multiply(new anchor.BN(777), new anchor.BN(888)).rpc();
  console.log("Your transaction signature", tx);
  })

  it("SQRT", async () => {
  const tx = await program.methods.sqrt(777).rpc();
  console.log("Your transaction signature", tx);
  })

  it("Log10", async () => {
  const tx = await program.methods.log10(777).rpc();
  console.log("Your transaction signature", tx);
});

// it("Catches an overflow op", async () => {
//   const tx = await program.methods.overflow(new anchor.BN(0), new anchor.BN(1)).rpc();
//   console.log("Your transaction signature", tx);
// });
});
