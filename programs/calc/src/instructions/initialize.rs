use anchor_lang::prelude::*;

use crate::constants::CALCULATOR_SEED;
use crate::state::Calculator;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + Calculator::INIT_SPACE,
        seeds = [CALCULATOR_SEED, user.key().as_ref()],
        bump,
    )]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let calculator = &mut ctx.accounts.calculator;
    calculator.authority = ctx.accounts.user.key();
    calculator.result = 0;
    msg!("Calculator initialized for {}", calculator.authority);
    Ok(())
}
