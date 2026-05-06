use anchor_lang::prelude::*;

#[error_code]
pub enum CalcError {
    #[msg("Division by zero is not allowed")]
    DivisionByZero,
    #[msg("Arithmetic overflow")]
    Overflow,
}
