use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("39rokqWKbrDvoaHtm6Z3Bfhn19ZgwpJGuuLTziy5bTxq");

#[program]
pub mod example_map {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, key1: u64, key2: u64, key3: u64) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Set>, key1: u64, key2: u64, key3: u64, val: u64) -> Result<()> {
        ctx.accounts.val.value = val;
        Ok(())
}
}

#[derive(Accounts)]
#[instruction(key1: u64, key2: u64, key3: u64)]
pub struct Initialize<'info> {

    #[account(init,
              payer = signer,
              space = size_of::<Val>() + 8,
              seeds=[&key1.to_le_bytes().as_ref(), &key2.to_le_bytes().as_ref(), &key3.to_le_bytes().as_ref()],
              bump)]
    val: Account<'info, Val>,
    
    #[account(mut)]
    signer: Signer<'info>,
    
    system_program: Program<'info, System>,
}

#[account]
pub struct Val {
    value: u64,
}

#[derive(Accounts)]
#[instruction(key1: u64, key2: u64, key3: u64)] 
pub struct Set<'info> {
    #[account(mut)]
    val: Account<'info, Val>,
}

