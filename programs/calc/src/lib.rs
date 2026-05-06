// Program modules: business logic is split across dedicated submodules.
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

// On-chain program address — must match the deployed keypair.
declare_id!("GpXJ6y5gn2wBaQTm7ZAuGrSgueGKKUfJ1MNGfZzu1rNe");

#[program]
pub mod calc {
    use super::*;

    /// Creates a new Calculator PDA account for the signing user, setting
    /// `authority` to the user's public key and `result` to 0.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    /// Computes `a + b` using checked arithmetic and stores the result in the
    /// caller's Calculator account. Returns `Overflow` if the sum exceeds i64.
    pub fn add(ctx: Context<Operation>, a: i64, b: i64) -> Result<()> {
        add::handler(ctx, a, b)
    }

    /// Computes `a - b` using checked arithmetic and stores the result in the
    /// caller's Calculator account. Returns `Overflow` if the difference
    /// underflows i64.
    pub fn subtract(ctx: Context<Operation>, a: i64, b: i64) -> Result<()> {
        subtract::handler(ctx, a, b)
    }

    /// Computes `a * b` using checked arithmetic and stores the result in the
    /// caller's Calculator account. Returns `Overflow` if the product exceeds
    /// i64.
    pub fn multiply(ctx: Context<Operation>, a: i64, b: i64) -> Result<()> {
        multiply::handler(ctx, a, b)
    }

    /// Computes `a / b` and stores the result in the caller's Calculator
    /// account. Returns `DivisionByZero` if `b` is 0, or `Overflow` for the
    /// i64::MIN / -1 edge case caught by `checked_div`.
    pub fn divide(ctx: Context<Operation>, a: i64, b: i64) -> Result<()> {
        divide::handler(ctx, a, b)
    }
}
