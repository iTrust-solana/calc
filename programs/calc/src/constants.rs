use anchor_lang::prelude::*;

// Seed prefix used to derive every user's Calculator PDA:
//   seeds = [CALCULATOR_SEED, user_pubkey]
#[constant]
pub const CALCULATOR_SEED: &[u8] = b"calculator";
