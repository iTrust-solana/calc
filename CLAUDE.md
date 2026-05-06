# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build the program (generates IDL + TS types)
anchor build

# Run all tests against a local validator
anchor test

# Run a single test file directly (validator must already be running)
npx ts-mocha -p ./tsconfig.json -t 1000000 tests/calc.ts

# Lint / format TypeScript and JS
npm run lint        # check
npm run lint:fix    # auto-fix

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

Rust toolchain is pinned to **1.89.0** (see `rust-toolchain.toml`). Anchor CLI **1.0.0** is required.

## Architecture

This is an Anchor (Solana) smart contract with a thin TypeScript test layer.

### On-chain program (`programs/calc/src/`)

- `lib.rs` — entrypoint; declares the program ID and dispatches each instruction to its handler module
- `state.rs` — `Calculator` account struct (`authority: Pubkey`, `result: i64`); derives `InitSpace` for space calculation
- `constants.rs` — `CALCULATOR_SEED` used in PDA derivation
- `error.rs` — custom error codes: `DivisionByZero` (6000), `Overflow` (6001)
- `instructions/operation.rs` — shared `Operation` accounts context (PDA + signer) reused by all four math instructions
- `instructions/initialize.rs` — creates the Calculator PDA with `init`
- `instructions/{add,subtract,multiply,divide}.rs` — each handler calls the checked Rust arithmetic method and maps errors to `CalcError::Overflow` / `CalcError::DivisionByZero`

### PDA scheme

Every user gets their own Calculator PDA derived from `["calculator", user_pubkey]`. The `Operation` context enforces `calculator.authority == user.key()`, so only the initializing wallet can mutate its own account.

### Test layer (`tests/calc.ts`)

Uses `@anchor-lang/core` (Anchor 1.x client). Tests derive the same PDA client-side and call instructions via `program.methods.<name>(...).accounts({ user }).rpc()`. The test suite runs sequentially and depends on state carried over between tests (each op reads the result written by the previous call).

### Program ID

`GpXJ6y5gn2wBaQTm7ZAuGrSgueGKKUfJ1MNGfZzu1rNe` — same for localnet and devnet (see `Anchor.toml`).
