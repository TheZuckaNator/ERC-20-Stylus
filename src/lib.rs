// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

mod erc20;
mod ownable2step;

// Alias the SDK's re-exports under the crate names expected by the macros.
use stylus_sdk::alloy_primitives as alloy_primitives;

use stylus_sdk::alloy_primitives::{Address, U256};
use stylus_sdk::prelude::*;
use crate::erc20::{Erc20, Erc20Params, Erc20Error};
use crate::ownable2step::{Ownable2Step, Ownable2StepError};

/// Immutable definitions
struct StylusTokenParams;
impl Erc20Params for StylusTokenParams {
    const NAME: &'static str = "StylusToken";
    const SYMBOL: &'static str = "STK";
    const DECIMALS: u8 = 18;
}

// Define the entrypoint as a Solidity storage object.
sol_storage! {
    #[entrypoint]
    struct StylusToken {
        /// Allows erc20 to access StylusToken's storage and make calls
        #[borrow]
        Erc20<StylusTokenParams> erc20;
        /// Ownable2Step for access control
        #[borrow]
        Ownable2Step ownable;
    }
}

// IMPORTANT: Only ONE #[public] block per contract!
#[public]
#[inherit(Erc20<StylusTokenParams>, Ownable2Step)]
impl StylusToken {
    /// Initialize the contract with an owner
    /// This should be called right after deployment
    pub fn init(&mut self, initial_owner: Address) -> Result<(), Ownable2StepError> {
        // Check if already initialized (owner is not zero)
        if self.ownable.owner() != Address::ZERO {
            return Err(Ownable2StepError::OwnableUnauthorizedAccount(
                ownable2step::OwnableUnauthorizedAccount { account: self.vm().msg_sender() },
            ));
        }
        self.ownable.init(initial_owner)?;
        Ok(())
    }

        /// Mints tokens to the caller - only owner can call
    pub fn mint(&mut self, value: U256) -> Result<(), Ownable2StepError> {
        // Check if caller is owner
        self.ownable.only_owner()?;
        
        // Mint tokens
        let sender: Address = self.vm().msg_sender();
        let _ = self.erc20.mint(sender, value);
        Ok(())
    }

    /// Mints tokens to another address - only owner can call
    pub fn mint_to(&mut self, to: Address, value: U256) -> Result<(), Ownable2StepError> {
        // Check if caller is owner
        self.ownable.only_owner()?;
        
        // Mint tokens
        let _ = self.erc20.mint(to, value);
        Ok(())
    }

    /// Burns tokens from the caller
    pub fn burn(&mut self, value: U256) -> Result<(), Erc20Error> {
        let sender: Address = self.vm().msg_sender();
        self.erc20.burn(sender, value)?;
        Ok(())
    }
}