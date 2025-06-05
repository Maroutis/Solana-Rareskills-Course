use anchor_lang::prelude::*;

declare_id!("9NwyaJFnng23cr5w4E7ZTcDAF8n183RW9D3BtLVkc9cN");

#[program]
pub mod deploy_tutorial {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        msg!("Test bytecode change2");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
