# Calc — On-chain Calculator (Solana / Anchor)

A Solana smart contract built with [Anchor](https://www.anchor-lang.com/) that exposes four arithmetic operations: **add**, **subtract**, **multiply**, and **divide**.

## Program ID

```
GpXJ6y5gn2wBaQTm7ZAuGrSgueGKKUfJ1MNGfZzu1rNe
```

## Features

- Four instructions: `add`, `subtract`, `multiply`, `divide`
- All operands are signed 64-bit integers (`i64`)
- Checked arithmetic — overflows return a custom `Overflow` error
- Division by zero returns a custom `DivisionByZero` error
- Per-user Calculator PDA account (seeds: `["calculator", user_pubkey]`)

## Prerequisites

| Tool | Version |
|---|---|
| Rust | 1.95+ |
| Solana CLI | 3.x |
| Anchor CLI | 1.0.0 |
| Node.js | 18+ |
| npm | 9+ |

## Project Structure

```
calc/
├── Anchor.toml
├── Cargo.toml
├── package.json
├── programs/calc/src/
│   ├── lib.rs              # Program entrypoint & instruction dispatch
│   ├── constants.rs        # PDA seed constant
│   ├── error.rs            # Custom error codes
│   ├── state.rs            # Calculator account struct
│   ├── instructions.rs     # Module declarations
│   └── instructions/
│       ├── initialize.rs   # Creates the Calculator PDA
│       ├── operation.rs    # Shared accounts struct for math ops
│       ├── add.rs          # a + b
│       ├── subtract.rs     # a - b
│       ├── multiply.rs     # a * b
│       └── divide.rs       # a / b (with zero-check)
└── tests/
    └── calc.ts             # Mocha/Chai integration tests
```

## Setup

```bash
# Clone and enter the project
cd calc

# Install JS dependencies
npm install
```

## Build

```bash
anchor build
```

This compiles the program and generates the IDL at `target/idl/calc.json` and TypeScript types at `target/types/calc.ts`.

## Test

```bash
anchor test
```

This spins up a local validator, deploys the program, and runs the test suite. Tests cover:

- Initialization of the Calculator account
- Addition (10 + 5 = 15)
- Subtraction (20 - 7 = 13)
- Multiplication (6 × 7 = 42)
- Division (100 / 4 = 25)
- Division by zero (expects `DivisionByZero` error)
- Negative number handling (-10 + 3 = -7)

## Deploy to Devnet

```bash
# 1. Make sure your Solana CLI is configured for devnet
solana config set --url https://api.devnet.solana.com

# 2. Airdrop SOL for deployment fees (if needed)
solana airdrop 2

# 3. Deploy
anchor deploy --provider.cluster devnet

# 4. Verify deployment
solana program show <PROGRAM_ID>
```

## Instructions Reference

### `initialize`

Creates the Calculator PDA account for the signing user.

**Accounts:**
- `calculator` — PDA (writable, init)
- `user` — Signer (writable, payer)
- `system_program` — System Program

### `add(a: i64, b: i64)`

Stores `a + b` in the calculator. Returns `Overflow` on overflow.

### `subtract(a: i64, b: i64)`

Stores `a - b` in the calculator. Returns `Overflow` on underflow.

### `multiply(a: i64, b: i64)`

Stores `a * b` in the calculator. Returns `Overflow` on overflow.

### `divide(a: i64, b: i64)`

Stores `a / b` in the calculator. Returns `DivisionByZero` if `b == 0`.

**Shared accounts for add/subtract/multiply/divide:**
- `calculator` — PDA (writable)
- `user` — Signer (must be the authority that initialized the calculator)

## Error Codes

| Code | Name | Message |
|---|---|---|
| 6000 | `DivisionByZero` | Division by zero is not allowed |
| 6001 | `Overflow` | Arithmetic overflow |

## License

ISC
