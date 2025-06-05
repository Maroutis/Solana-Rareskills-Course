use anchor_lang::prelude::*;

declare_id!("HVTx7cWRDTt4rQEDNzhNsyKeQEAdiS8ttAwULgYJaHiZ");

#[program]
pub mod read {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
