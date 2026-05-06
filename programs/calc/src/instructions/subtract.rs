use anchor_lang::prelude::*;

use crate::error::CalcError;
use crate::instructions::operation::Operation;

/// Subtracts `b` from `a`, stores the result in the Calculator account, and
/// logs it. `checked_sub` returns `None` on i64 underflow, which is mapped to
/// `CalcError::Overflow` and causes the transaction to fail.
pub fn handler(ctx: Context<Operation>, a: i64, b: i64) -> Result<()> {
    let calculator = &mut ctx.accounts.calculator;
    calculator.result = a.checked_sub(b).ok_or(CalcError::Overflow)?;
    msg!("{} - {} = {}", a, b, calculator.result);
    Ok(())
}
