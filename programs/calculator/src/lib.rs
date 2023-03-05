use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;



declare_id!("CsQYgrrqQoSwBUQF8ro7aFJgzyhjpUymbiMBeRFiv4Db");

#[program]
pub mod calculator {
    use super::*;

    pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok({})
    }

    pub fn add(ctx: Context<Addition>, num1: i128, num2: i128) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        Ok(())
    }
    pub fn subtract(ctx: Context<Subtraction>, num1: i128, num2: i128) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 - num2;
        Ok(())
    }
    pub fn multiply(ctx: Context<Multiplication>, num1: i128, num2: i128) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 * num2;
        Ok(())
    }

    pub fn divide(ctx: Context<Division>, num1: i128, num2: i128) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
      //ensure!(num2 != 0, "Cannot divide by zero"); //Dont know how to resolve this issue yet
        calculator.result = num1 / num2;
        Ok(())
    }
    pub fn power(ctx: Context<Exponentiation>, num1: i128, num2: i128) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1.pow(num2 as u32);
        Ok(())
        }
    
}

#[derive(Accounts)]
pub struct Create<'info> {

    #[account(init, payer=user, space=264)]
    pub calculator: Account<'info, Calculator>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,

}
#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Subtraction<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Multiplication<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Division<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}
#[derive(Accounts)]
pub struct Exponentiation<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}
#[account]
pub struct Calculator {
    greeting: String,
    result: i128
}



