use anchor_lang::prelude::*;

/// Custom error codes returned by the calc program.
/// Anchor assigns sequential discriminants starting at 6000.
#[error_code]
pub enum CalcError {
    /// Returned by `divide` when the divisor `b` is 0. Code 6000.
    #[msg("Division by zero is not allowed")]
    DivisionByZero,
    /// Returned by any arithmetic instruction when the result cannot be
    /// represented as i64 (overflow or underflow). Code 6001.
    #[msg("Arithmetic overflow")]
    Overflow,
}
