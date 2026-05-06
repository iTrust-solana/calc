use anchor_lang::prelude::*;

/// On-chain account that stores per-user calculator state.
/// Each user has exactly one Calculator PDA derived from their public key.
#[account]
#[derive(InitSpace)] // auto-computes INIT_SPACE for space = 8 + Calculator::INIT_SPACE
pub struct Calculator {
    /// The wallet that initialized this account; only this signer may call
    /// arithmetic instructions on it.
    pub authority: Pubkey,
    /// The result of the most recent arithmetic instruction. Starts at 0.
    pub result: i64,
}