use anchor_lang::prelude::*;
 
#[derive(InitSpace)] // This derive-macro inspects your Escrow struct’s fields and computes a pub const INIT_SPACE: usize that equals exactly the Borsh-serialized size of your data. You then use that in your #[account(init, space = …)] to allocate the right number of bytes on-chain. so that:
// Escrow::INIT_SPACE gives you the exact space needed for the data after the 8-byte discriminator, and
#[account(discriminator = 1)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
    pub bump: u8,
}