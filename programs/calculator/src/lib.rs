use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;



declare_id!("CsQYgrrqQoSwBUQF8ro7aFJgzyhjpUymbiMBeRFiv4Db");

#[error_code]
pub enum ErrorCode {

    #[msg("Division by zero")]
    DivisionByZero = 1,

    #[msg("Overflow or underflow error")]
    OverflowUnderflow = 2,

}
impl From<ErrorCode> for ProgramError {
    fn from(e: ErrorCode) -> Self {
        match e {
            ErrorCode::DivisionByZero => ProgramError::Custom(1),
            ErrorCode::OverflowUnderflow => ProgramError::Custom(2),
        }
    }
}



#[program]
pub mod calculator {
    use super::*;

    pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok({})
    }

    pub fn add(ctx: Context<Operation>, num1: i128, num2: i128) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        match num1.checked_add(num2) {
            Some(result) => {
                calculator.result = result;
                Ok(())
            } 
            None => Err(ErrorCode::OverflowUnderflow.into()),
        }
    }
    pub fn subtract(ctx: Context<Operation>, num1: i128, num2: i128) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        match num1.checked_sub(num2) {
            Some(result) => {
                calculator.result = result;
                Ok(())
            }
            None => Err(ErrorCode::OverflowUnderflow.into()),
        }
    }
    pub fn multiply(ctx: Context<Operation>, num1: i128, num2: i128) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        match num1.checked_mul(num2) {
            Some(result) => {
                calculator.result = result;
                Ok(())
            }
            None => Err(ErrorCode::OverflowUnderflow.into()),
        }
    }

    pub fn divide(ctx: Context<Operation>, num1: i128, num2: i128) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        if num2 == 0 {
            return Err(ErrorCode::DivisionByZero.into());
        }
        match num1.checked_div(num2) {
            Some(result) => {
                calculator.result = result;
                Ok(())
            }
            None => Err(ErrorCode::OverflowUnderflow.into()),
        }
    }

    pub fn power(ctx: Context<Operation>, num1: i128, num2: i128) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        match num1.checked_pow(num2 as u32) {
            Some(result) => {
                calculator.result = result;
                Ok(())
            }
            None => Err(ErrorCode::OverflowUnderflow.into()),
        }
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
pub struct Operation<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[account]
pub struct Calculator {
    greeting: String,
    result: i128
}



