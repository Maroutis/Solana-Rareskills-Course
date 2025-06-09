use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{         // replaces raw Account<'_, Mint>/TokenAccount
  TokenInterface,                // for `Program<'_, TokenInterface>`
  Mint,                          // now from token_interface
  TokenAccount,                  // now from token_interface
  transfer_checked,              // CPI helper
  TransferChecked,
};
use anchor_lang::prelude::*;
use crate::state::Escrow;
use crate::errors::EscrowError;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        init,
        payer = maker,
        space = Escrow::INIT_SPACE + Escrow::DISCRIMINATOR.len(),
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,
 
    /// Token Accounts
    #[account(
        mint::token_program = token_program // they’re part of Anchor’s constraint language. Think of it as a little domain-specific language (DSL) that lives inside the #[account(..)] attribute, where each constraint is written as:
        // <namespace>::<constraint_name> = <value>
        // Look at the def of namespce below

    )]
    pub mint_a: InterfaceAccount<'info, Mint>, // InterfaceAccount<'info, Mint> tells anchor to Treat this account’s data as an SPL‐Token Mint” (i.e. it must match the in‐memory layout of the Mint struct https://docs.rs/spl-token/7.0.0/src/spl_token/state.rs.html#18)
    #[account(
        mint::token_program = token_program // In Anchor, whenever you want to do a CPI (cross‐program invocation) into some on‐chain program, you have to pass that program’s ID as one of the accounts in your Context<…>. For SPL-Token, that on‐chain program is the “Token Program” (the BPF binary at
        // TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
        // Anchor gives you a convenient alias for it called token_program
        // mint::token_program = token_program tells Anchor, “verify that the mint_a account is actually owned by that SPL-Token program.” Under the hood it does:
        // require!(
        // mint_a.to_account_info().owner == token_program.key(),
        // ConstraintOwner
        // ); 
        // this specific pattern only applies to SPL Mint token interface InterfaceAccount<'info, Mint>

        // SPL-Token Program (Tokenkeg…)
        // ↔ A single, shared ERC-20 library/VM that all tokens use => TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
        // Mint Account (MINT_PUBKEY)
        // ↔ The instance of that library—i.e. an ERC-20 contract’s storage+state.
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>, // https://docs.rs/cronos-anchor-spl/latest/src/cronos_anchor_spl/token.rs.html#350 and https://docs.rs/spl-token/7.0.0/src/spl_token/state.rs.html#89
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
 
    /// Programs
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>, // https://docs.rs/anchor-spl/latest/src/anchor_spl/token_interface.rs.html#71
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    fn populate_escrow(&mut self, seed: u64, amount: u64, bump: u8) -> Result<()> {
        self.escrow.set_inner(Escrow {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive: amount,
            bump,
        });
 
        Ok(())
    }
 
    fn deposit_tokens(&self, amount: u64) -> Result<()> {
        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.maker_ata_a.to_account_info(),
                    mint: self.mint_a.to_account_info(),
                    to: self.vault.to_account_info(),
                    authority: self.maker.to_account_info(),
                },
            ),
            amount,
            self.mint_a.decimals
        )?;
 
        Ok(())
    }
}
 
pub fn handler(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
    // Validate the amount
    require!(receive > 0, EscrowError::InvalidAmount);
    require!(amount > 0, EscrowError::InvalidAmount);
 
    // Save the Escrow Data
    ctx.accounts.populate_escrow(seed, receive, ctx.bumps.escrow)?;
 
    // Deposit Tokens
    ctx.accounts.deposit_tokens(amount)?;
 
    Ok(())
}


// A namespace is simply a way to group related constraints together. Anchor defines several namespaces of constraints:

// mint — constraints that only make sense on a token Mint account, e.g.

// mint::decimals = 6

// mint::token_program = token_program

// mint::authority = some_pubkey

// associated_token — constraints for an associated‐token‐account (ATA), e.g.

// associated_token::mint = mint_a

// associated_token::authority = owner

// associated_token::token_program = token_program

// seeds — constraints around PDAs (just one namespace)

// token — constraints for a TokenAccount, etc.


// Within each namespace Anchor knows which fields to check. So
// mint::token_program = token_program
// means:
// “When I verify mint_a’s account, I will assert
// mint_a.owner == token_program.key()



// system_program is the built-in Solana program (11111111111111111111111111111111) that handles native SOL operations and account management (creating accounts, allocating space, assigning ownership to programs, transferring lamports).

// token_program is the one and only SPL-Token program (TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA) that handles all fungible-token operations (minting, burning, transfers, approvals) for every SPL-Token mint on the chain.