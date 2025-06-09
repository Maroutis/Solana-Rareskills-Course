use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{             // replaces raw Account<'_, Mint>/TokenAccount
  TokenInterface,                // for `Program<'_, TokenInterface>`
  Mint,                          // now from token_interface
  TokenAccount,                  // now from token_interface
  transfer_checked,              // CPI helper
  TransferChecked,
  close_account,
  CloseAccount,
};
use anchor_lang::prelude::*;
use crate::state::Escrow;
use crate::errors::EscrowError;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>, // type validating that the account is owned by the system account https://docs.rs/anchor-lang/latest/src/anchor_lang/accounts/system_account.rs.html#14-16
    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        has_one = maker @ EscrowError::InvalidMaker, //maker of escrow same as maker of Take
        has_one = mint_a @ EscrowError::InvalidMintA,
        has_one = mint_b @ EscrowError::InvalidMintB,
    )]
    pub escrow: Box<Account<'info, Escrow>>,// stack and heapboth exist only for the duration of the program's execution. They dont persist. On Solana every program starts with only 4 KB of stack space. If you had a large account type (or many account types) sitting directly in your context, you could easily blow past that limit and get a stack‐overflow at runtime. Example:
    //     #[account]
    // pub struct Escrow {
    //     // imagine a lot of fields here, or a big vector
    //     pub data: [u8; 1024],

    // Putting Account<'_, Escrow> directly into your Take<'info> struct would allocate the entire Escrow data on the stack when the account context is built. In contrast: pub escrow: Box<Account<'info, Escrow>>, only puts an 8- or 16-byte “box pointer” on the stack; the full Escrow account data lives in heap memory (which comes from Solana’s account data allocator, not the BPF stack).
 
    /// Token Accounts
    pub mint_a: Box<InterfaceAccount<'info, Mint>>,
    pub mint_b: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,
 
    /// Programs
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    fn transfer_to_maker(&mut self) -> Result<()> {
        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.taker_ata_b.to_account_info(),
                    to: self.maker_ata_b.to_account_info(),
                    mint: self.mint_b.to_account_info(),
                    authority: self.taker.to_account_info(),
                },
            ),
            self.escrow.receive,
            self.mint_b.decimals
        )?;
 
        Ok(())
    }
 
    fn withdraw_and_close_vault(&mut self) -> Result<()> {
        // Create the signer seeds for the Vault
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];
 
        // Transfer Token A (Vault -> Taker)
        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.vault.to_account_info(),
                    to: self.taker_ata_a.to_account_info(),
                    mint: self.mint_a.to_account_info(),
                    authority: self.escrow.to_account_info(),
                },
                &signer_seeds
            ),
            self.vault.amount,
            self.mint_a.decimals
        )?;
 
        // Close the Vault
        close_account(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                CloseAccount {
                    account: self.vault.to_account_info(),
                    authority: self.escrow.to_account_info(),
                    destination: self.maker.to_account_info(),
                },
                &signer_seeds
            ),
        )?;
 
        Ok(())
    }
}
 
pub fn handler(ctx: Context<Take>) -> Result<()> {
    // Transfer Token B to Maker
    ctx.accounts.transfer_to_maker()?;
 
    // Withdraw and close the Vault
    ctx.accounts.withdraw_and_close_vault()?;
 
    Ok(())
}