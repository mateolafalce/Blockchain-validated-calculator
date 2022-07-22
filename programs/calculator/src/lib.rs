use anchor_lang::prelude::*;

declare_id!("CVtb3RNs78UqttMd2Wmr33r7k5fo9bv5uAHBMpnqw8Fi");

#[program]
pub mod calculator {
    use super::*;

    pub fn add(ctx: Context<Initialize>,
        number1: u16,
        number2: u16
    ) -> Result<()> {
        let x = &mut ctx.accounts.numbers;
        x.number1 = number1; x.number2 = number2;
        msg!("The result of the sum is {}", x.number1 + x.number2);
        Ok(())
    }
    pub fn div(ctx: Context<Initialize>,
        number1: u16,
        number2: u16
    ) -> Result<()> {
        let x = &mut ctx.accounts.numbers;
        x.number1 = number1; x.number2 = number2;
        msg!("The result of the division is {}", x.number1 / x.number2);
        Ok(())
    }
    pub fn sub(ctx: Context<Initialize>,
        number1: u16,
        number2: u16
    ) -> Result<()> {
        let x = &mut ctx.accounts.numbers;
        x.number1 = number1; x.number2 = number2;
        msg!("The result of the subtraction is {}", x.number1 - x.number2);
        Ok(())
    }
    pub fn mul(ctx: Context<Initialize>,
        number1: u16,
        number2: u16
    ) -> Result<()> {
        let x = &mut ctx.accounts.numbers;
        x.number1 = number1; x.number2 = number2;
        msg!("The result of the multiplication is {}", x.number1 * x.number2);
        Ok(())
    }
    pub fn squa(ctx: Context<Initialize>,
        number1: u16
    ) -> Result<()> {
        let x = &mut ctx.accounts.numbers;
        x.number1 = number1;
        msg!("The result of the square is {}", x.number1 * x.number1);
        Ok(())
    }
    pub fn perc(ctx: Context<Initialize>,
        number1: u16,
        number2: u16
    ) -> Result<()> {
        let x = &mut ctx.accounts.numbers;
        x.number1 = number1; x.number2 = number2;
        msg!("The result of the percentage is {}", x.number2 * (x.number1 / 100));
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = DataStore::LEN)]
    pub numbers: Account<'info, DataStore>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataStore {
    number1: u16,
    number2: u16,
}

impl DataStore {
    const LEN: usize = DISCRIMINATOR 
    + NUMBERS;
}

const DISCRIMINATOR: usize = 8;
const NUMBERS: usize = 2 * 2;

#[error_code]
pub enum ErrorCode {
    #[msg("Your number is too big for a character u64")]
    TooLong
}