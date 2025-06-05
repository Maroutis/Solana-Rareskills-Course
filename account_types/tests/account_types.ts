
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AccountTypes } from "../target/types/account_types";

describe("account_types", () => {  
    const wallet = anchor.workspace.AccountTypes.provider.wallet;  
	
    // Configure the client to use the local cluster.  
    anchor.setProvider(anchor.AnchorProvider.env());  

    const program = anchor.workspace.AccountTypes as Program<AccountTypes>;  
    it("Load account with accountInfo", async () => {    
        // CREATE AN ACCOUNT NOT OWNED BY THE PROGRAM    
        const newKeypair = anchor.web3.Keypair.generate();    
        const tx = new anchor.web3.Transaction().add(      
            anchor.web3.SystemProgram.createAccount({        
                fromPubkey: wallet.publicKey,        
                newAccountPubkey: newKeypair.publicKey,        
                space: 16,        
                lamports: await anchor          
                    .getProvider()          				
                    .connection
                    .getMinimumBalanceForRentExemption(32),        		
                programId: program.programId,      
            })    
	);    

	await anchor.web3.sendAndConfirmTransaction(      
            anchor.getProvider().connection,      
            tx,      
            [wallet.payer, newKeypair]    
	);    

	// READ THE DATA IN THE ACCOUNT    
	await program.methods      
            .foo()      
            .accounts({ someAccount: newKeypair.publicKey })      
            .rpc();  
    });


    it("Wrong owner with Account", async () => {    
          await program.methods.hello().rpc();  
      });
});
