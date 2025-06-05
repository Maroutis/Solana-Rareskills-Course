import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Batch } from "../target/types/batch";

describe("batch", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Batch as Program<Batch>;

  it("Set the number to 5, initializing if necessary", async () => {
    const wallet = anchor.workspace.Batch.provider.wallet.payer;
    const [pda, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);

    // console.log the address of the pda
    console.log(pda.toBase58());

    let accountInfo = await anchor.getProvider().connection.getAccountInfo(pda);

    console.log("Account", accountInfo);

    let transaction = new anchor.web3.Transaction();
    if (accountInfo == null || accountInfo.lamports == 0 || accountInfo.owner == anchor.web3.SystemProgram.programId) {
      console.log("need to initialize");
      const initTx = await program.methods.initialize().accounts({pda: pda}).transaction(); //@note using .transaction() instead of .rpc() to create a tx.
      transaction.add(initTx);
    }
    else {
      console.log("no need to initialize");
    }

    // for u32, we don't need to use big numbers
    const setTx = await program.methods.set(5).accounts({pda: pda}).transaction();
    transaction.add(setTx);

    await anchor.web3.sendAndConfirmTransaction(anchor.getProvider().connection, transaction, [wallet]);

    const pdaAcc = await program.account.pda.fetch(pda);
    console.log(pdaAcc.value); // prints 5
  });
});

