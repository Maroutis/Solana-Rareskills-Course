use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("ASJfuW5CvjZzXeJ3nqtxgr3Z4T3MrvtoDe9BtXvpXCBX");

const STARTING_POINTS: u32 = 10;

#[program]
pub mod points {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.player.points = STARTING_POINTS;
        ctx.accounts.player.authority = ctx.accounts.signer.key();
        Ok(())
    }

    pub fn transfer_points(ctx: Context<TransferPoints>,
                           amount: u32) -> Result<()> {
        // require!(ctx.accounts.from.authority == ctx.accounts.signer.key(), Errors::SignerIsNotAuthority);
        // require!(ctx.accounts.from.points >= amount, Errors::InsufficientPoints);

        ctx.accounts.from.points -= amount;
        ctx.accounts.to.points += amount;
        Ok(())
    }
}

#[error_code]
pub enum Errors {
    #[msg("SignerIsNotAuthority")]
    SignerIsNotAuthority,
    #[msg("InsufficientPoints")]
    InsufficientPoints
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,
              payer = signer,
              space = size_of::<Player>() + 8,
              seeds = [&(signer.as_ref().key().to_bytes())],
              bump)]
    player: Account<'info, Player>,
    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u32)] // anchor constraint must be passed as an instruction
pub struct TransferPoints<'info> {
    #[account(mut, has_one = authority @ Errors::SignerIsNotAuthority, constraint = from.points >= amount @ Errors::InsufficientPoints)] // no need for require
    from: Account<'info, Player>,
    #[account(mut)]
    to: Account<'info, Player>,
    authority: Signer<'info>, // this account must appear as a signer
}

#[account]
pub struct Player {
    points: u32,
    authority: Pubkey
}