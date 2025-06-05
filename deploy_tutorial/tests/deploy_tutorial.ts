import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";


import fs from 'fs'
let idl = JSON.parse(fs.readFileSync('target/idl/deploy_tutorial.json', 'utf-8'))

describe("deployed", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // Change this to be your programID
  const programID = "9NwyaJFnng23cr5w4E7ZTcDAF8n183RW9D3BtLVkc9cN";
  const program = new Program(idl, anchor.getProvider());

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
