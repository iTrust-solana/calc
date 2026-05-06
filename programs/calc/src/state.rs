use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Calculator {
    pub authority: Pubkey,
    pub result: i64,
}