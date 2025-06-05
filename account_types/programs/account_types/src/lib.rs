use anchor_lang::prelude::*;

declare_id!("8d3rrmerx5n2ZRKkBKRPE4HgRWuyRiNYY4GCZuYtKryp");

#[program]
pub mod account_types {    
    use super::*;    

    pub fn foo(ctx: Context<Foo>) -> Result<()> {        
        let data = &ctx.accounts.some_account.try_borrow_data()?;        
        msg!("{:?}", data);        
        Ok(())    
    }

        pub fn hello(ctx: Context<Hello>) -> Result<()> {        
        let lamports = ctx.accounts.signer.lamports();        
        let address = &ctx.accounts
            .signer
            .signer_key().unwrap();        
        msg!(
            "hello {:?} you have {} lamports", 
            address, 
            lamports
        );        
        Ok(())    
    }
}

#[derive(Accounts)]
pub struct Foo<'info> {    
    /// CHECK: we are just printing the data    
    some_account: AccountInfo<'info>,
}


#[derive(Accounts)]
pub struct Hello<'info> {    
    pub signer: Signer<'info>,
}