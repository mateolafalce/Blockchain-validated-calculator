use anchor_lang::prelude::*;

declare_id!("8UmDMaghFgLMnQXZXYnNJb2Uxpk1NUHMBo5ZbkeFYmwA");

#[program]
pub mod calculator {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>, 
        autoridad: Pubkey
    ) -> Result<()> {

        let number = &mut ctx.accounts.datastore;

        number.autoridad = autoridad;

        number.add1 = 23;
        number.add2 = 34;
        number.division1 = 34;
        number.division2 = 98;

        pub fn abc(num1: u64, num2: u64) {
            let c: u64 = num1 + num2;
            msg!("The result of the sum is {}", c);
        }
        pub fn def(num1: u64, num2: u64){
            let c: u64 = num1 / num2;
            msg!("The result of the division is {}", c);  
        }

        abc(number.add1, number.add2);
        def(number.division1, number.division2);
    Ok(())
}
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 30)]
    pub datastore: Account<'info, DataStore>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataStore {
    add1: u64,
    add2: u64,
    division1: u64,
    division2: u64,
    autoridad: Pubkey
}