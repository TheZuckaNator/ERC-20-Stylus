//! ERC-20 example (unaudited)

use alloc::string::String;
use core::marker::PhantomData;

// Alias the SDK’s re-exports for the macros.
use stylus_sdk::alloy_primitives as alloy_primitives;

use stylus_sdk::alloy_primitives::{Address, U256};
use stylus_sdk::alloy_sol_types::sol;
use stylus_sdk::prelude::*; // #[public], sol_storage!, log, Host, etc.

pub trait Erc20Params {
    const NAME: &'static str;
    const SYMBOL: &'static str;
    const DECIMALS: u8;
}

sol_storage! {
    pub struct Erc20<T> {
        mapping(address => uint256) balances;
        mapping(address => mapping(address => uint256)) allowances;
        uint256 total_supply;
        PhantomData<T> phantom;
    }
}

sol! {
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    error InsufficientBalance(address from, uint256 have, uint256 want);
    error InsufficientAllowance(address owner, address spender, uint256 have, uint256 want);
}

#[derive(SolidityError)]
pub enum Erc20Error {
    InsufficientBalance(InsufficientBalance),
    InsufficientAllowance(InsufficientAllowance),
}

impl<T: Erc20Params> Erc20<T> {
    fn vm_host(&self) -> &dyn Host { self.vm() }

    pub fn _transfer(&mut self, from: Address, to: Address, value: U256) -> Result<(), Erc20Error> {
        // debit
        let mut sender_balance = self.balances.setter(from);
        let have = sender_balance.get();
        if have < value {
            return Err(Erc20Error::InsufficientBalance(InsufficientBalance {
                from, have, want: value,
            }));
        }
        sender_balance.set(have - value);

        // credit
        let mut to_balance = self.balances.setter(to);
        let to_old = to_balance.get();
        to_balance.set(to_old + value);

        // event
        log(self.vm_host(), Transfer { from, to, value });
        Ok(())
    }

    pub fn mint(&mut self, address: Address, value: U256) -> Result<(), Erc20Error> {
        let mut balance = self.balances.setter(address);
        let old = balance.get();
        balance.set(old + value);

        self.total_supply.set(self.total_supply.get() + value);

        log(self.vm_host(), Transfer { from: Address::ZERO, to: address, value });
        Ok(())
    }

    pub fn burn(&mut self, address: Address, value: U256) -> Result<(), Erc20Error> {
        let mut balance = self.balances.setter(address);
        let have = balance.get();
        if have < value {
            return Err(Erc20Error::InsufficientBalance(InsufficientBalance {
                from: address, have, want: value,
            }));
        }
        balance.set(have - value);

        self.total_supply.set(self.total_supply.get() - value);

        log(self.vm_host(), Transfer { from: address, to: Address::ZERO, value });
        Ok(())
    }
}

#[public]
impl<T: Erc20Params> Erc20<T> {
    pub fn name() -> String { T::NAME.into() }
    pub fn symbol() -> String { T::SYMBOL.into() }
    pub fn decimals() -> u8 { T::DECIMALS }

    pub fn total_supply(&self) -> U256 { self.total_supply.get() }

    pub fn balance_of(&self, owner: Address) -> U256 { self.balances.get(owner) }

    pub fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Erc20Error> {
        let sender = self.vm().msg_sender();
        self._transfer(sender, to, value)?;
        Ok(true)
    }

    pub fn transfer_from(&mut self, from: Address, to: Address, value: U256) -> Result<bool, Erc20Error> {
        let sender = self.vm().msg_sender();

        let mut sender_allowances = self.allowances.setter(from);
        let mut allowance = sender_allowances.setter(sender);
        let have = allowance.get();
        if have < value {
            return Err(Erc20Error::InsufficientAllowance(InsufficientAllowance {
                owner: from, spender: sender, have, want: value,
            }));
        }
        allowance.set(have - value);

        self._transfer(from, to, value)?;
        Ok(true)
    }

    pub fn approve(&mut self, spender: Address, value: U256) -> bool {
        let sender = self.vm().msg_sender();
        self.allowances.setter(sender).insert(spender, value);
        log(self.vm_host(), Approval { owner: sender, spender, value });
        true
    }

    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.allowances.getter(owner).get(spender)
    }
}
