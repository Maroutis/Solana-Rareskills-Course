use anchor_lang::prelude::*;

declare_id!("5BdWzpUUw3cVfzyrmNMobKq84PXEjtBHNA8Fkwkn4P9a");

#[program]
pub mod day1 {
    use super::*;

    pub fn initialize2(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!"); // **** NEW LINE HERE ****
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
