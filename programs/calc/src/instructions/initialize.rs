use anchor_lang::prelude::*;

use crate::constants::CALCULATOR_SEED;
use crate::state::Calculator;

/// Accounts required to initialize a new Calculator PDA.
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// The Calculator PDA being created. Derived from [CALCULATOR_SEED, user].
    /// Anchor allocates and zero-initializes the account; `user` pays rent.
    #[account(
        init,
        payer = user,
        space = 8 + Calculator::INIT_SPACE,
        seeds = [CALCULATOR_SEED, user.key().as_ref()],
        bump,
    )]
    pub calculator: Account<'info, Calculator>,
    /// The wallet creating the calculator; signs the transaction and pays rent.
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Sets `calculator.authority` to the user's public key and `result` to 0,
/// then logs the new account owner. Called exactly once per wallet.
pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let calculator = &mut ctx.accounts.calculator;
    calculator.authority = ctx.accounts.user.key();
    calculator.result = 0;
    msg!("Calculator initialized for {}", calculator.authority);
    Ok(())
}
