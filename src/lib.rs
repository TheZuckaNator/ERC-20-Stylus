// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

mod erc20;

// Alias the SDK’s re-exports under the crate names expected by the macros.
use stylus_sdk::alloy_primitives as alloy_primitives;

use stylus_sdk::alloy_primitives::{Address, U256};
use stylus_sdk::prelude::*;
use crate::erc20::{Erc20, Erc20Params, Erc20Error};

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
    }
}

#[public]
#[inherit(Erc20<StylusTokenParams>)]
impl StylusToken {
    /// Mints tokens to the caller
    pub fn mint(&mut self, value: U256) -> Result<(), Erc20Error> {
        let sender: Address = self.vm().msg_sender();
        self.erc20.mint(sender, value)?;
        Ok(())
    }

    /// Mints tokens to another address
    pub fn mint_to(&mut self, to: Address, value: U256) -> Result<(), Erc20Error> {
        self.erc20.mint(to, value)?;
        Ok(())
    }

    /// Burns tokens from the caller
    pub fn burn(&mut self, value: U256) -> Result<(), Erc20Error> {
        let sender: Address = self.vm().msg_sender();
        self.erc20.burn(sender, value)?;
        Ok(())
    }
}
