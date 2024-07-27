use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("8G5pZpKZdB8Q5uoSJAog1NYqDvnCMZjxif52FRQeUcKN");

#[program]
pub mod mycalcdapp {

    use super::*;

    pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }
    pub fn add(ctx: Context<Addition>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result =num1+num2;
        Ok(())
    }
    pub fn multiply(ctx: Context<Multiplication>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1*num2;
        Ok(())
    }
    pub fn subtract(ctx: Context<Subtraction>, num1: i64, num2: i64) -> ProgramResult {

    }
    pub fn divide(ctx: Context<Division>, num1: i64, num2: i64) -> ProgramResult {}
}

#[derive(Accounts)]
pub struct Addition<'info>{
    #[account(mut)]
    pub calculator : Account<'info,Calculator>,
}
#[derive(Accounts)]
pub struct Multiplication<'info>{
    #[account(mut)]
    pub calculator : Account<'info,Calculator>,
}

#[account]
pub struct Calculator{
    pub greeting : String,
    pub result :i64,
    pub remainder: i64,
}