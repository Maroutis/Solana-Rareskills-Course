use anchor_lang::prelude::*;

declare_id!("AxY7KWP5m1SgmXEL21UrbKsQ4cFFEmC4Kd6VwpQowBYi");

#[program]
pub mod owner {
    use super::*;

    pub fn initialize_keypair(ctx: Context<InitializeKeypair>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_pda(ctx: Context<InitializePda>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeKeypair<'info> {
    #[account(init, payer = signer, space = 8)]
    keypair: Account<'info, Keypair>,
    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializePda<'info> {
    #[account(init, payer = signer, space = 8, seeds = [], bump)]
    pda: Account<'info, Pda>,
    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[account]
pub struct Keypair();

#[account]
pub struct Pda();