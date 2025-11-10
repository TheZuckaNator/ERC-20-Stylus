// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec; // some Stylus macros expect Vec in scope

use alloy_sol_types::sol;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{
    alloy_primitives::{address, Address, U256},
    prelude::*,
};

sol_interface! {
    interface IERC20 {
        function balanceOf(address owner) external view returns (uint256);
        function transfer(address to, uint256 value) external returns (bool);
    }
}

const ADDRESSES: [Address; 2] = [
    address!("5E1497dD1f08C87b2d8FE23e9AAB6c1De833D927"),
    address!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E"),
];

// Define some persistent storage using the Solidity ABI.
// `PushBasedDistributor` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct PushBasedDistributor {}
}

// Declare events and Solidity error types
sol! {
    error TransferFailed();
    error InsufficientBalance();
}

#[derive(SolidityError)]
pub enum Error {
    TransferFailed(TransferFailed),
    InsufficientBalance(InsufficientBalance),
}

// Manual Debug impl so Result<_, Error>::unwrap() works in tests
impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::TransferFailed(_) => write!(f, "TransferFailed"),
            Error::InsufficientBalance(_) => write!(f, "InsufficientBalance"),
        }
    }
}

#[public]
impl PushBasedDistributor {
    /// Pure helper: given a balance, compute the share per recipient.
    /// This is what we unit test in normal Rust tests.
    pub fn compute_share(balance: U256) -> Result<U256, Error> {
        let recipient_count = ADDRESSES.len();

        if balance <= U256::from(0) {
            return Err(Error::InsufficientBalance(InsufficientBalance {}));
        }

        Ok(balance / U256::from(recipient_count))
    }

    /// Calculates the amount to distribute per recipient by reading the token balance.
    fn get_amount_to_distribute(&self, token_addr: Address) -> Result<U256, Error> {
        // Get the contract's token balance
        let token_instance = IERC20::new(token_addr);
        let balance: U256 = token_instance
            .balance_of(self, self.vm().contract_address())
            .unwrap();

        // Delegate to the pure helper
        Self::compute_share(balance)
    }

    /// Distributes tokens to all predefined addresses.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_share_with_balance() {
        // 1000 / 2 = 500
        let res = PushBasedDistributor::compute_share(U256::from(1000));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), U256::from(500));
    }

    #[test]
    fn test_compute_share_with_zero_balance() {
        let res = PushBasedDistributor::compute_share(U256::from(0));
        assert!(res.is_err());
    }

    #[test]
    fn test_compute_share_with_odd_amount() {
        // 1001 / 2 = 500 (integer division)
        let res = PushBasedDistributor::compute_share(U256::from(1001));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), U256::from(500));
    }

    #[test]
    fn test_compute_share_various_amounts() {
        let cases = [
            (U256::from(100), U256::from(50)),
            (U256::from(1000), U256::from(500)),
            (U256::from(99), U256::from(49)),
        ];

        for (total, expected) in cases {
            let res = PushBasedDistributor::compute_share(total).unwrap();
            assert_eq!(
                res, expected,
                "Failed for total {}, expected {}, got {}",
                total, expected, res
            );
        }
    }
}
