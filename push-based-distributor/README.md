# Push-Based Distributor (Stylus)

This is a simple **push-based ERC20 token distributor** written in Rust for the Arbitrum Stylus VM.

The contract holds a balance of an ERC20 token and, when `distribute` is called, it splits the balance evenly across a fixed list of recipient addresses.

---

## Overview

- **Language:** Rust (`no_std`, Stylus)
- **VM:** Arbitrum Stylus
- **Pattern:** Push-based distribution (contract sends tokens out)
- **Recipients:** Hard-coded address list in the contract

Key pieces:

- `ADDRESSES`: static list of recipient `Address`es.
- `compute_share(balance: U256) -> Result<U256, Error>`  
  Pure helper that computes how much each recipient should get.
- `get_amount_to_distribute(token_addr: Address) -> Result<U256, Error>`  
  Reads the contract’s ERC20 balance and returns the per-recipient amount.
- `distribute(token: IERC20) -> Result<(), Error>`  
  Calls `transfer` to each recipient with the computed share.

---

## Contract Logic

```rust
const ADDRESSES: [Address; 2] = [
    address!("5E1497dD1f08C87b2d8FE23e9AAB6c1De833D927"),
    address!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E"),
];
```

## Errors

```rust
sol! {
    error TransferFailed();
    error InsufficientBalance();
}

#[derive(SolidityError)]
pub enum Error {
    TransferFailed(TransferFailed),
    InsufficientBalance(InsufficientBalance),
}
```


## Core Functions

``rust
#[public]
impl PushBasedDistributor {
    /// Pure helper: given a balance, compute the share per recipient.
    pub fn compute_share(balance: U256) -> Result<U256, Error> {
        let recipient_count = ADDRESSES.len();

        if balance <= U256::from(0) {
            return Err(Error::InsufficientBalance(InsufficientBalance {}));
        }

        Ok(balance / U256::from(recipient_count))
    }

    /// Reads token balance and returns per-recipient amount.
    fn get_amount_to_distribute(&self, token_addr: Address) -> Result<U256, Error> {
        let token_instance = IERC20::new(token_addr);
        let balance: U256 = token_instance
            .balance_of(self, self.vm().contract_address())
            .unwrap();

        Self::compute_share(balance)
    }

    /// Distributes tokens evenly to the static ADDRESSES list.
    pub fn distribute(&mut self, token: IERC20) -> Result<(), Error> {
        let amount = self.get_amount_to_distribute(token.address.clone())?;

        for &recipient in ADDRESSES.iter() {
            if !token
                .transfer(&mut *self, recipient, amount)
                .unwrap_or(false)
            {
                return Err(Error::TransferFailed(TransferFailed {}));
            }
        }

        Ok(())
    }
}
```

## Requirements
Rust toolchain (1.75+ is usually fine; match your Stylus template)

### cargo installed

Stylus SDK / template set up (e.g. cloned from stylus-hello-world or similar)

cargo stylus plugin if you want to export ABI / deploy

### Building & Testing
Run tests (pure Rust)

```bash
cargo test
```

### These tests only exercise the pure helper:

* compute_share with various balances

* Zero/negative balance paths (error)

*  No Stylus VM or external calls are used in tests, so they run as normal Rust unit tests.

*  Exporting ABI (for deployment)
If you have cargo stylus installed and the export-abi feature wired up, you can export the ABI with:

``bash
cargo stylus export-abi
```

This will generate ABI artifacts for deployment / tooling.

* Note: Depending on your setup, this may live behind a feature flag like --features export-abi.

How to Use On-Chain

High-level flow:

Deploy PushBasedDistributor to Stylus.

Send ERC20 tokens to the distributor contract address.

Call distribute(tokenAddress) with the ERC20 you want to distribute:

### The contract:

Reads its current token balance.

Computes share = balance / ADDRESSES.len().

Pushes that share to each address in ADDRESSES.

Any remainder from integer division stays in the contract.

### Notes / Extensions
Things you might want to add later:

Make ADDRESSES configurable (owner-set recipients).

Support multiple token types with per-token config.

Emit events when distributions happen.

Add access control (e.g. only owner / role can call distribute).
``

