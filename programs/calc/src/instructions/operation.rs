use anchor_lang::prelude::*;

use crate::constants::CALCULATOR_SEED;
use crate::state::Calculator;

/// Shared account context reused by all four arithmetic instructions
/// (add, subtract, multiply, divide).
#[derive(Accounts)]
pub struct Operation<'info> {
    /// The caller's Calculator PDA, verified by PDA seeds and the authority
    /// constraint so only the initializing wallet can mutate this account.
    #[account(
        mut,
        seeds = [CALCULATOR_SEED, user.key().as_ref()],
        bump,
        constraint = calculator.authority == user.key(),
    )]
    pub calculator: Account<'info, Calculator>,
    /// The wallet that owns this calculator; must sign every operation.
    pub user: Signer<'info>,
}
