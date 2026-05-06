pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("GpXJ6y5gn2wBaQTm7ZAuGrSgueGKKUfJ1MNGfZzu1rNe");

#[program]
pub mod calc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn add(ctx: Context<Operation>, a: i64, b: i64) -> Result<()> {
        add::handler(ctx, a, b)
    }

    pub fn subtract(ctx: Context<Operation>, a: i64, b: i64) -> Result<()> {
        subtract::handler(ctx, a, b)
    }

    pub fn multiply(ctx: Context<Operation>, a: i64, b: i64) -> Result<()> {
        multiply::handler(ctx, a, b)
    }

    pub fn divide(ctx: Context<Operation>, a: i64, b: i64) -> Result<()> {
        divide::handler(ctx, a, b)
    }
}
