use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        pub fn add(a: i64, b: i64) {
            let c: i64 = a + b;
            msg!("The result of the sum is {}", c);
        }
        pub fn subtraction(a: i64, b: i64){
            let c: i64 = a - b;
            msg!("The result of the subtraction is {}", c);  
        }
        pub fn multiplication(a: i64, b: i64){
            let c: i64 = a * b;
            msg!("The result of the multiplication is {}", c);  
        }
        pub fn division(a: i64, b: i64){
            let c: i64 = a / b;
            msg!("The result of the division is {}", c);  
        }

        add(2, 56);
        subtraction(45, 6);
        multiplication(23, 7);
        division(56, 2);
    Ok(())
}
}

#[derive(Accounts)]
pub struct Initialize {}
