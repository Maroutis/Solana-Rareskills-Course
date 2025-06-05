use anchor_lang::prelude::*;

declare_id!("DXrNppawEESq2aGaUBDgFzMjz9NxV7guQeEEcqHHDsAE");

#[program]
pub mod sysvars {
    use super::*;
    use anchor_lang::solana_program::sysvar::{instructions, last_restart_slot, fees::Fees, recent_blockhashes::RecentBlockhashes};

    pub fn initialize(ctx: Context<Initialize>, number: u32) -> Result<()> {
        // Get the Clock sysvar
        let clock = Clock::get()?;

        msg!(
            "clock: {:?}",
            // Retrieve all the details of the Clock sysvar
            clock
        );

        // Get the EpochSchedule sysvar
        let epoch_schedule = EpochSchedule::get()?;

        msg!(
            "epoch schedule: {:?}",
            // Retrieve all the details of the EpochSchedule sysvar
            epoch_schedule
        );

        // Get the Rent sysvar
        let rent_var = Rent::get()?;
        msg!(
            "Rent {:?}",
            // Retrieve all the details of the Rent sysvar
            rent_var
        );

        // Accessing the StakeHistory sysvar
        // Create an array to store the StakeHistory account
        let arr = [ctx.accounts.stake_history.clone()];

        // Create an iterator for the array
        let accounts_iter = &mut arr.iter();

        // Get the next account info from the iterator (still StakeHistory)
        let sh_sysvar_info = next_account_info(accounts_iter)?;

        // Create a StakeHistory instance from the account info
        let stake_history = StakeHistory::from_account_info(sh_sysvar_info)?;

        msg!("stake_history: {:?}", stake_history);

        // Get Instruction sysvar
        let arr = [ctx.accounts.instruction_sysvar.clone()];

        let account_info_iter = &mut arr.iter();

        let instructions_sysvar_account = next_account_info(account_info_iter)?;

        // Load the instruction details from the instruction sysvar account
        let instruction_details =
            instructions::load_instruction_at_checked(0, instructions_sysvar_account)?;

        msg!(
            "Instruction details of this transaction: {:?}",
            instruction_details
        );
        msg!("Number is: {}", number);

        let lrs_details =
            last_restart_slot::LastRestartSlot::get()?;

        msg!(
            "The last restart slot is: {:?}",
            lrs_details
        );

        // other method
        let arr = [ctx.accounts.last_restart_slot.clone()];
        let info_iter = &mut arr.iter();
        let lrs_account_info = next_account_info(info_iter)?;

        let lrs = last_restart_slot::LastRestartSlot::from_account_info(lrs_account_info)?;

        msg!("LastRestartSlot sysvar: {:?}", lrs);

        Ok(())
    }


}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK:
    pub stake_history: AccountInfo<'info>, // We create an account for the StakeHistory sysvar
    /// CHECK:
    pub recent_blockhashes: AccountInfo<'info>,
    /// CHECK:
    pub instruction_sysvar: AccountInfo<'info>,
    /// CHECK:
    pub last_restart_slot: AccountInfo<'info>,
}