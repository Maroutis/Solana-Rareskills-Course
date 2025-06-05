use anchor_lang::prelude::*;

declare_id!("CDT7mtYzpfbDcPzYUATzdQkCeiopzZJxyNAruNGpjeSe");

const OWNER: &str = "EbBpJhNj6XNRhZFVokobLpce7brHcr9X5pE9EHpHhoHH";

#[program]
pub mod day14 {
    use super::*;

    #[access_control(check(&ctx))] // executes the given access control method before running the main instruction.
    pub fn initialize(ctx: Context<OnlyOwner>) -> Result<()> {
        let the_signer1: &mut Signer = &mut ctx.accounts.signer1;
        let the_signer2: &mut Signer = &mut ctx.accounts.signer2;

        msg!("The signer1: {:?}", *the_signer1.key);
        msg!("The signer2: {:?}", *the_signer2.key);

        Ok(())
    }
}

fn check(ctx: &Context<OnlyOwner>) -> Result<()> {
    // Check if signer === owner
    require_keys_eq!(
        ctx.accounts.signer1.key(),
        OWNER.parse::<Pubkey>().unwrap(),
        OnlyOwnerError::NotOwner
    );

    Ok(())
}

#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    pub signer1: Signer<'info>,
    pub signer2: Signer<'info>,
}

// #[derive(Accounts)]
// pub struct OnlyOwner<'info> {
//     signer_account: Signer<'info>,
// }

// An enum for custom error codes
#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call this function!")]
    NotOwner,
}