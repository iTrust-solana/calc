use anchor_lang::prelude::*;

use crate::constants::CALCULATOR_SEED;
use crate::state::Calculator;

#[derive(Accounts)]
pub struct Operation<'info> {
    #[account(
        mut,
        seeds = [CALCULATOR_SEED, user.key().as_ref()],
        bump,
        constraint = calculator.authority == user.key(),
    )]
    pub calculator: Account<'info, Calculator>,
    pub user: Signer<'info>,
}
