use anchor_lang::prelude::*;

declare_id!("3E7U3DA7oLxDS4LN7fVpLfouuYka4jSuoq2a4pdLkVux");

#[program]
pub mod calculator {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>, 
        autoridad: Pubkey,
        add1: u64,add2: u64,
        div1: u64,div2: u64,
        sub1: u64,sub2: u64,
        mul1: u64,mul2: u64,
        squ:u64,
        per: u64,
        numtoper: u64
    ) -> Result<()> {

        let number = &mut ctx.accounts.datastore;
        let clock = Clock::get().unwrap();

        number.autoridad = autoridad;
        number.tiempo = clock.unix_timestamp;

        number.add1 = add1;number.add2 = add2;
        number.div1 = div1;number.div2 = div2;
        number.sub1 = sub1;number.sub2 = sub2;
        number.mul1 = mul1;number.mul2 = mul2;
        number.squ = squ;number.per = per; 
        number.numtoper = numtoper;

        fn add(num1: u64, num2: u64) {
            let c: f64 = (num1 + num2) as f64;
            msg!("The result of the sum is {}", c);
        }
        fn div(num1: u64, num2: u64){
            let c: f64 = (num1 / num2) as f64;
            msg!("The result of the division is {}", c);  
        }
        fn sub(num1: u64, num2: u64){
            let c: f64 = (num1 - num2) as f64;
            msg!("The result of the subtraction is {}", c);  
        }
        fn mul(num1: u64, num2: u64){
            let c: f64 = (num1 * num2) as f64;
            msg!("The result of the multiplication is {}", c);  
        }
        fn squa(a: u64){
            let c: f64 = (a * a) as f64;
            msg!("The number squared is {}", c);
        }
        fn perc(a: u64, b: u64){
            let hundred: u64 = 100;
            let c: f64 = (b * (a / hundred)) as f64;
            msg!("the percentage is {}", c);
        }

        add(number.add1, number.add2);
        div(number.div1, number.div2);
        sub(number.sub1, number.sub2);
        mul(number.mul1, number.mul2);
        squa(number.squ);
        perc(number.per, number.numtoper);

        msg!("Autor: {}", number.autoridad);
        msg!("Hora: {}", number.tiempo);
    Ok(())
}
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = DataStore::LEN)]
    pub datastore: Account<'info, DataStore>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataStore {
    add1: u64,add2: u64,
    div1: u64,div2: u64,
    sub1: u64,sub2: u64,
    mul1: u64,mul2: u64,
    squ: u64,
    per: u64, 
    numtoper: u64,
    tiempo: i64,
    autoridad: Pubkey
}

impl DataStore {
    const LEN: usize = DISCRIMINATOR 
    + PUBLIC_KEY 
    + TIEMPO
    + NUMBERS;
}

const DISCRIMINATOR: usize = 8;
const PUBLIC_KEY: usize = 32;
const NUMBERS: usize = 8 * 11;
const TIEMPO: usize = 8;

#[error_code]
pub enum ErrorCode {
    #[msg("Your number is too big for a character u64")]
    TooLong
}