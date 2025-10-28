# ERC-20 Token with Stylus - Homework

## What's this about?

You're gonna build an ERC-20 token using Stylus (that's Arbitrum's way of writing smart contracts in Rust instead of Solidity). The contract is already mostly done - you just need to fill in a few missing pieces.


## Project files

```
.
├── src/
│   ├── lib.rs          # Main contract - you'll edit this
│   └── erc20.rs        # ERC-20 stuff - also needs edits
├── scripts/
│   ├── .env.example    # Copy this to .env
│   ├── deploy.sh       # Deploys your contract
│   └── test.sh         # Tests everything
└── README.md           # You're reading it
```

### Setup project

```bash
# Go to your project folder
cd your-project-folder

# Check if Rust is working
rustc --version
cargo --version
```

### Configure your .env file

```bash
cd scripts
cp .env.example .env
nano .env  # or whatever editor you like
```

Put these in your `.env`:
```env
PRIVATE_KEY=your_wallet_private_key
RPC_URL=https://sepolia-rollup.arbitrum.io/rpc
CONTRACT_ADDRESS=  # leave this empty for now
```

## The actual homework

There are 3 blanks you need to fill. That's it.

### Blank #1: The struct name

Find where it says `impl /* 1. ______ */` and put your struct name there.

```rust
// You'll have something like this at the top:
pub struct MyToken {
    // stuff...
}

// So fill it in like:
impl MyToken {
    // your functions...
}
```

Just look for how the struct is named and use that same name.

---

### Blank #2: Making functions public

This one's easy. Just put `pub` before the function.

```rust
// Change this:
/* 2. ______ */ fn transfer(&mut self, to: Address, amount: U256) -> bool {
    // code
}

// To this:
pub fn transfer(&mut self, to: Address, amount: U256) -> bool {
    // code
}
```

You need `pub` so people can actually call your functions from outside.

---

### Blank #3: The number type

Token amounts need to be big numbers, so use `U256`.

```rust
// Fill this:
pub fn mint(&mut self, to: Address, value: /* 3. ______ */) {
    // code
}

// Like this:
pub fn mint(&mut self, to: Address, value: U256) {
    // code
}
```

Why U256? Because tokens can have crazy large amounts. Think billions of tokens with 18 decimals.

---

## How to do it

### 1. Fill in the blanks

Open `src/lib.rs` and `src/erc20.rs` and complete all three spots.

### 2. Build it

```bash
cargo build --release --target wasm32-unknown-unknown
```

If it compiles, you probably did it right!

## Deploy your contract

### Run the deploy script

```bash
./scripts/deploy.sh
```

You'll see something like:
```
Compiling contract...
Deploying to Arbitrum Sepolia...
Contract deployed at: 0x1234567890abcdef...
```

### Update your .env

Copy that contract address and paste it in your `.env`:

```env
CONTRACT_ADDRESS=0x1234567890abcdef...
```

## Test it

```bash
./scripts/test.sh
```

This checks if everything works:
- Token name and symbol
- Total supply
- Balances
- Transfers
- Approvals
- The whole thing

You should see:
```
Running tests...
✓ Test 1: Get token name
✓ Test 2: Get token symbol
✓ Test 3: Check total supply
✓ Test 4: Transfer tokens
✓ Test 5: Approve spender
✓ Test 6: TransferFrom
All tests passed!
```

## Stuff you should know

### What's ERC-20?

It's just a standard way to make tokens. Every ERC-20 token has these functions:

```rust
// Reading stuff
fn name() -> String
fn symbol() -> String
fn decimals() -> u8
fn total_supply() -> U256
fn balance_of(owner: Address) -> U256
fn allowance(owner: Address, spender: Address) -> U256

// Doing stuff
fn transfer(to: Address, amount: U256) -> bool
fn approve(spender: Address, amount: U256) -> bool
fn transfer_from(from: Address, to: Address, amount: U256) -> bool
```

### Rust types vs Solidity

| Solidity | Rust | What it is |
|----------|------|------------|
| `address` | `Address` | Wallet address |
| `uint256` | `U256` | Big number |
| `bool` | `bool` | true/false |
| `string` | `String` | Text |
| `mapping` | `StorageMap` | Like a dictionary |

### Stylus quirks

- `&mut self` - for functions that change stuff
- `&self` - for functions that just read
- `StorageMap` - where you save data on the blockchain

## When things break

### Build errors

**Can't find type Address?**
```rust
// Add this at the top
use stylus_sdk::prelude::*;
```

**Weird pub keyword error?**
```rust
// Don't do this:
impl pub MyToken { }

// Do this:
impl MyToken {
    pub fn transfer(...) { }
}
```


## IssuesRuint & Alloy Compatibility Issues

While building the Stylus ERC-20 contract, I encountered several dependency conflicts between ruint, alloy, and stylus-sdk:

**Const-Evaluation Panic (E0080)**

The compiler threw an error: BYTES must be equal to Self::BYTES.

This came from ruint 1.17.0, which added a new const check incompatible with the version of alloy-primitives bundled in Stylus SDK.

Fix: Pinned ruint to version 1.16.0.

Multiple Alloy Versions (0.8.x vs 1.x)

Cargo automatically fetched both alloy 0.8.x and 1.x, leading to type conflicts (Address and U256 types didn’t match).

Fix: Locked both alloy-primitives and alloy-sol-types to version 0.8.20, which matches Stylus SDK’s dependency tree.

Missing Crate Errors (no external crate alloy_sol_types)

The macros sol! and sol_storage! expand to ::alloy_sol_types, requiring the crate to exist explicitly in Cargo.toml.

Initially, I relied only on Stylus’s re-exports, which wasn’t enough.


Final Working Dependencies
[dependencies]
stylus-sdk        = "0.9.2"
alloy-primitives   = "=0.8.20"
alloy-sol-types    = "=0.8.20"
ruint              = "=1.16.0"


These versions resolved all compile-time panics, macro errors, and dependency mismatches.