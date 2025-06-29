use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("5917S87QDp1MkTCcUdJCe1gppYRciqAsoMcV4bQCyNKH");

#[program]
pub mod keypair_vs_pda {
    use super::*;

    pub fn initialize_keypair_account(ctx: Context<InitializeKeypairAccount>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeKeypairAccount<'info> {
    // This is the program derived address
    #[account(init,
              payer = signer,
              space = size_of::<MyKeypairAccount>() + 8,)]
    pub my_keypair_account: Account<'info, MyKeypairAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyKeypairAccount {
    x: u64,
}