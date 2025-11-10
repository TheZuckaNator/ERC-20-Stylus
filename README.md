# ERC-20 Token + Push-Based Distributor (Stylus Homework)

## 🧠 What's this about?

This project combines two Stylus smart contracts written in Rust:

1. **ERC-20 Token** — a standard token implementation written in Stylus.  
2. **Push-Based Distributor** — a companion contract that automatically splits and sends ERC-20 tokens to a list of predefined addresses.

You’ll learn how to:
- Implement a basic ERC-20 token in Rust using Stylus.
- Deploy both contracts to Arbitrum Stylus.
- Automatically distribute your token to multiple recipients.

---

## 📂 Project structure

```yaml

Day-1/
├── erc20/
│ ├── src/
│ │ ├── lib.rs # ERC-20 logic (your main editing target)
│ │ └── erc20.rs # ERC-20 implementation helpers
│ ├── scripts/
│ │ ├── .env.example
│ │ ├── deploy.sh
│ │ └── test.sh
│ └── Cargo.toml
│
├── push-based-distributor/
│ ├── src/
│ │ └── lib.rs # Push-based distributor logic (already working)
│ ├── Cargo.toml
│ └── README.md # Detailed explanation for the distributor
│
└── README.md # You’re reading this file
```


---

## ⚙️ Setup

```bash
cd ~/Stylus/Day-1
rustc --version
cargo --version
```

Both should work before continuing.

🔐 Configure your .env
```bash
cd erc20/scripts
cp .env.example .env
nano .env
```

## Then fill in:

env

```
PRIVATE_KEY=your_wallet_private_key
RPC_URL=https://sepolia-rollup.arbitrum.io/rpc
CONTRACT_ADDRESS=  # leave this empty until deployment
```

🧩 ERC-20 Homework
You’ll fill in three blanks inside the ERC-20 code:

1️⃣ Struct name
Replace the placeholder in the impl block with your token’s struct name.

```rust
impl MyToken {
    // functions go here
}
```
## 2 Make functions public

Add pub to functions that should be callable externally:

```rust
pub fn transfer(&mut self, to: Address, amount: U256) -> bool {
    // ...
}
```
### 3 Use the correct type (U256)

All balances and token values should use U256:

```rust
pub fn mint(&mut self, to: Address, value: U256) {
    // ...
}
```

## Build
``` bash
cargo build --release --target wasm32-unknown-unknown
```

If it compiles, you’re good!

### Deploy your ERC-20 contract
```bash
./scripts/deploy.sh
```

You should see output like:

```css
Compiling contract...
Deploying to Arbitrum Sepolia...
Contract deployed at: 0x1234567890abcdef...
```

Then update your .env:

```env
CONTRACT_ADDRESS=0x1234567890abcdef...
```

 Test your token
```bash

./scripts/test.sh
```

Expected output:

```yaml
Running tests...
✓ Test 1: Get token name
✓ Test 2: Get token symbol
✓ Test 3: Check total supply
✓ Test 4: Transfer tokens
✓ Test 5: Approve spender
✓ Test 6: TransferFrom
```
* Push-Based Distributor
Once your ERC-20 works, check out the companion project:

📄 push-based-distributor/README.md

The Push-Based Distributor is a Stylus contract that automatically splits the token balance it holds and sends equal shares to a list of predefined wallet addresses.

How it works:

The contract imports your deployed ERC-20 interface (IERC20).

When distribute() is called:

It checks its current token balance.

Calculates the per-recipient share.

Sends each recipient their portion.

Any leftover tokens (due to rounding) remain in the contract.

File:
push-based-distributor/src/lib.rs

The distributor uses:

```rust
const ADDRESSES: [Address; 2] = [
    address!("5E1497dD1f08C87b2d8FE23e9AAB6c1De833D927"),
    address!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E"),
];
```

You can easily modify that list to fit your use case.

🧰 Key Concepts
ERC-20 Standard Functions
```rust
// View functions
fn name() -> String
fn symbol() -> String
fn decimals() -> u8
fn total_supply() -> U256
fn balance_of(owner: Address) -> U256
fn allowance(owner: Address, spender: Address) -> U256

// Mutating functions
fn transfer(to: Address, amount: U256) -> bool
fn approve(spender: Address, amount: U256) -> bool
fn transfer_from(from: Address, to: Address, amount: U256) -> bool
```

### Stylus Quirks
Concept	Explanation
&mut self	For functions that modify contract state
&self	For read-only functions
StorageMap	Persistent on-chain key/value store
Address, U256	Imported from stylus_sdk::prelude::*

Dependency Notes (ruint + alloy compatibility)
While building the ERC-20 contract, there were dependency version conflicts between ruint, alloy, and stylus-sdk.


```toml
[dependencies]
stylus-sdk        = "0.9.2"
alloy-primitives   = "=0.8.20"
alloy-sol-types    = "=0.8.20"
ruint              = "=1.16.0"
```

This combination removes all compile-time panics and macro expansion errors.

