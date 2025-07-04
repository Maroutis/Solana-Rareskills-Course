use anchor_lang::prelude::*;
 
mod state;
mod errors;
mod instructions;

use instructions::make::*;
use instructions::take::*;
use instructions::refund::*;

declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod blueshift_anchor_escrow {
    use super::*;
  
    #[instruction(discriminator = 0)] // you’re telling Anchor “instead of hashing my function name, just use the little-endian u64 0 as my 8-byte prefix” (i.e. [0,0,0,0,0,0,0,0]). Similarly 1 turns into [1,0,0,0,0,0,0,0].
    pub fn make(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
        instructions::make::handler(ctx, seed, receive, amount)?;
        Ok(())
    }
 
    #[instruction(discriminator = 1)]
    pub fn take(ctx: Context<Take>) -> Result<()> {
        instructions::take::handler(ctx)?;
        Ok(())
    }
 
    #[instruction(discriminator = 2)]
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        instructions::refund::handler(ctx)?;
        Ok(())
    }
}