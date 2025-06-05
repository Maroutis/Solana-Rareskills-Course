import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { KeypairVsPda } from "../target/types/keypair_vs_pda";

// this airdrops sol to an address
async function airdropSol(publicKey, amount) {
  let airdropTx = await anchor.getProvider().connection.requestAirdrop(publicKey, amount * anchor.web3.LAMPORTS_PER_SOL);
  await confirmTransaction(airdropTx);
}

async function confirmTransaction(tx) {
  const latestBlockHash = await anchor.getProvider().connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: tx,
  });
}

describe("keypair_vs_pda", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.KeypairVsPda as Program<KeypairVsPda>;

  it("Is initialized -- keypair version", async () => {
		
    const newKeypair = anchor.web3.Keypair.generate();
    await airdropSol(newKeypair.publicKey, 1e9); // 1 SOL

    const secondKeypair = anchor.web3.Keypair.generate();
    await airdropSol(secondKeypair.publicKey, 1e9); // 1 SOL

    console.log("the keypair account address is", newKeypair.publicKey.toBase58());

    const seeds = []
    const [pda, _bump] = anchor
                        .web3
                        .PublicKey
                        .findProgramAddressSync(
                            seeds,
                            program.programId);


    // not going to work
    await program.methods.initializeKeypairAccount()
      .accounts({myKeypairAccount: pda})
      .signers([secondKeypair]) // the signer must be the keypair
      .rpc();
  });

  it("Writing to keypair account fails", async () => {
    const newKeypair = anchor.web3.Keypair.generate();
    var recieverWallet = anchor.web3.Keypair.generate();

    await airdropSol(newKeypair.publicKey, 10);

    var transaction = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: newKeypair.publicKey,
        toPubkey: recieverWallet.publicKey,
        lamports: 1 * anchor.web3.LAMPORTS_PER_SOL,
      }),
    );
    await anchor.web3.sendAndConfirmTransaction(anchor.getProvider().connection, transaction, [newKeypair]);
    console.log('sent 1 lamport') 

    await program.methods.initializeKeypairAccount()
      .accounts({myKeypairAccount: newKeypair.publicKey})
      .signers([newKeypair]) // the signer must be the keypair
      .rpc();
      
    console.log("initialized");
    // try to transfer again, this fails. This account is now owned by the program.
    var transaction = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: newKeypair.publicKey,
        toPubkey: recieverWallet.publicKey,
        lamports: 1 * anchor.web3.LAMPORTS_PER_SOL,
      }),
    );
    await anchor.web3.sendAndConfirmTransaction(anchor.getProvider().connection, transaction, [newKeypair]);
  });

  it("Console log account owner", async () => {

    console.log(`The program address is ${program.programId}`) 
    const newKeypair = anchor.web3.Keypair.generate();
    var recieverWallet = anchor.web3.Keypair.generate();

    // get account owner before initialization
    await airdropSol(newKeypair.publicKey, 10);
    const accountInfoBefore = await anchor.getProvider().connection.getAccountInfo(newKeypair.publicKey);
    console.log(`initial keypair account owner is ${accountInfoBefore.owner}`);

    await program.methods.initializeKeypairAccount()
      .accounts({myKeypairAccount: newKeypair.publicKey})
      .signers([newKeypair]) // the signer must be the keypair
      .rpc();
      
    // get account owner after initialization
	
    const accountInfoAfter = await anchor.getProvider().connection.getAccountInfo(newKeypair.publicKey);
    console.log(`initial keypair account owner is ${accountInfoAfter.owner}`);
  });
});

