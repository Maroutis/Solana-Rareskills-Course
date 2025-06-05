import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BasicStorage } from "../target/types/basic_storage";

describe("basic_storage", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BasicStorage as Program<BasicStorage>;

  it("Is initialized!", async () => {
    const seeds = []
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);

    console.log("the storage account address is", myStorage.toBase58());

    await program.methods.initialize().accounts({ myStorage: myStorage }).rpc();

    await program.methods.set(new anchor.BN(170)).accounts({ myStorage: myStorage }).rpc();

    await program.methods.printX().accounts({myStorage: myStorage}).rpc();

    await program.methods.readAndIncrement().accounts({myStorage: myStorage}).rpc();

    // ***********************************
    // *** NEW CODE TO READ THE STRUCT ***
    // ***********************************
    let myStorageStruct = await program.account.myStorage.fetch(myStorage);
    console.log("The value of x is:",myStorageStruct.x.toString());

    });

});

