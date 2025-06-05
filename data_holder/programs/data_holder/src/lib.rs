use anchor_lang::prelude::*;
use std::mem::size_of; 

declare_id!("8yaCKq7dSTBMQbahva6MvmZuUcPMixw6rWqFZMHWNVT5");

#[program]
pub mod data_holder {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, base: u64, exponent: u32) -> Result<()> {
        ctx.accounts.storage.x = u64::pow(base, exponent);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // This is the program derived address
    #[account(init,
              payer = signer,
              space=size_of::<Storage>() + 8,
              seeds = [],
              bump)]
    pub storage: Account<'info, Storage>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Storage {
    x: u64,
}