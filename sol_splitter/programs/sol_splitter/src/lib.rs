use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("BjDXeo34WMdpYUAhUQr1ECCfF6m2nnMWFpGmnPSggACt");

#[program]
pub mod sol_splitter {
    use super::*;

    // 'a, 'b, 'c are Rust lifetimes, ignore them for now
    pub fn split_sol<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, SplitSol<'info>>,
        amount: u64,
    ) -> Result<()> {

        // let cpi_context1 = CpiContext::new(
        //     ctx.accounts.system_program.to_account_info(), 

        //     system_program::Transfer {
        //         from: ctx.accounts.signer.to_account_info(),
        //         to: ctx.accounts.recipient1.to_account_info(),
        //     }
        // );

        // let cpi_context2 = CpiContext::new(
        //     ctx.accounts.system_program.to_account_info(), 

        //     system_program::Transfer {
        //         from: ctx.accounts.signer.to_account_info(),
        //         to: ctx.accounts.recipient2.to_account_info(),
        //     }
        // );

        // let res1 = system_program::transfer(cpi_context1, amount/2);

        // let res2 = system_program::transfer(cpi_context2, amount/2);

        // if res1.is_ok() && res2.is_ok() {
        //     return Ok(());
        // } else {
        //     return err!(Errors::TransferFailed);
        // }
        let amount_each_gets = amount / ctx.remaining_accounts.len() as u64;
        let system_program = &ctx.accounts.system_program;

        // note the keyword `remaining_accounts`
        for recipient in ctx.remaining_accounts {
            let cpi_accounts = system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: recipient.to_account_info(),
            };
            let cpi_program = system_program.to_account_info();
            let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

            let res = system_program::transfer(cpi_context, amount_each_gets);
            if !res.is_ok() {
                return err!(Errors::TransferFailed);
            }
        }

        Ok(())
    }
}

#[error_code]
pub enum Errors {
    #[msg("transfer failed")]
    TransferFailed,
}

#[derive(Accounts)]
pub struct SendSol<'info> {
    /// CHECK: we do not read or write the data of this account
    #[account(mut)]
    recipient1: UncheckedAccount<'info>,

    /// CHECK: we do not read or write the data of this account
    #[account(mut)]
    recipient2: UncheckedAccount<'info>,
    
    system_program: Program<'info, System>,

    #[account(mut)]
    signer: Signer<'info>,
}


#[derive(Accounts)]
pub struct SplitSol<'info> {
    // /// CHECK: we do not read or write the data of this account
    // #[account(mut)]
    // recipient1: UncheckedAccount<'info>,

    // /// CHECK: we do not read or write the data of this account
    // #[account(mut)]
    // recipient2: UncheckedAccount<'info>,
    
    system_program: Program<'info, System>,

    #[account(mut)]
    signer: Signer<'info>,
}