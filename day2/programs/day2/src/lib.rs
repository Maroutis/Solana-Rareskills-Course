use anchor_lang::prelude::*;

declare_id!("Czt61gF6DJxTGiDHChG9PhJVUu76DNdibvPidFzr9fGc");

#[program]
pub mod day2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,
                      a: u64,
                      b: u64,
                      message: String) -> Result<()> {
        msg!("You said {:?}", message);
        msg!("You sent {} and {}", a, b);
        Ok(())
    }

    pub fn array(ctx: Context<Initialize>,
             arr: Vec<u64>) -> Result<()> {
    msg!("Your array {:?}", arr);
    Ok(())

    }

    pub fn overflow(ctx: Context<Initialize>, a:u64, b: u64) -> Result<()> {
        let c = a - b;
        msg!("c = {}", c);
        let result = a.checked_sub(b).unwrap();
        Ok(())
    }

    pub fn add(ctx: Context<Initialize>, a:u64, b: u64) -> Result<()> {
        let result = a.checked_add(b).unwrap();
        msg!("{} + {}  = {}", a, b, result);
        Ok(())
    }

    pub fn sub(ctx: Context<Initialize>, a:u64, b: u64) -> Result<()> {
        let result = a.checked_sub(b).unwrap();
        msg!("{} - {}  = {}", a, b, result);
        Ok(())
    }

    pub fn multiply(ctx: Context<Initialize>, a:u64, b: u64) -> Result<()> {
        let result = a.checked_mul(b).unwrap();
        msg!("{} x {}  = {}", a, b, result);
        Ok(())
    }

    pub fn sqrt(ctx: Context<Initialize>, a:f64) -> Result<()> {
        let result = a.sqrt();
        msg!("sqrt({}) = {}",a, result);
        Ok(())
    }

    pub fn log10(ctx: Context<Initialize>, a:f64) -> Result<()> {
        let result = a.log(10.0);
        msg!("log10({}) = {}",a, result);
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize {}
