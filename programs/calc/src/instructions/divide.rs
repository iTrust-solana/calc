use anchor_lang::prelude::*;

use crate::error::CalcError;
use crate::instructions::operation::Operation;

/// Divides `a` by `b`, stores the result in the Calculator account, and logs
/// it. Fails with `DivisionByZero` before attempting the division when `b` is
/// 0. `checked_div` also catches the i64::MIN / -1 overflow edge case,
/// returning `CalcError::Overflow` for that.
pub fn handler(ctx: Context<Operation>, a: i64, b: i64) -> Result<()> {
    require!(b != 0, CalcError::DivisionByZero);

    let calculator = &mut ctx.accounts.calculator;
    calculator.result = a.checked_div(b).ok_or(CalcError::Overflow)?;
    msg!("{} / {} = {}", a, b, calculator.result);
    Ok(())
}
