use anchor_lang::prelude::*;

use crate::error::CalcError;
use crate::instructions::operation::Operation;

pub fn handler(ctx: Context<Operation>, a: i64, b: i64) -> Result<()> {
    let calculator = &mut ctx.accounts.calculator;
    calculator.result = a.checked_mul(b).ok_or(CalcError::Overflow)?;
    msg!("{} * {} = {}", a, b, calculator.result);
    Ok(())
}
