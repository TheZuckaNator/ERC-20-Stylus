//! Ownable2Step implementation for Stylus

use stylus_sdk::alloy_primitives::Address;
use stylus_sdk::alloy_sol_types::sol;
use stylus_sdk::prelude::*;
use stylus_sdk::evm;

sol_storage! {
    pub struct Ownable2Step {
        address owner;
        address pending_owner;
    }
}

sol! {
    event OwnershipTransferred(address indexed previous_owner, address indexed new_owner);
    event OwnershipTransferStarted(address indexed previous_owner, address indexed new_owner);

    error OwnableUnauthorizedAccount(address account);
    error OwnableInvalidOwner(address owner);
}

#[derive(SolidityError)]
pub enum Ownable2StepError {
    OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
    OwnableInvalidOwner(OwnableInvalidOwner),
}

impl Ownable2Step {
    /// Initialize the owner (call this in your contract's constructor logic)
    pub fn init(&mut self, initial_owner: Address) -> Result<(), Ownable2StepError> {
        if initial_owner == Address::ZERO {
            return Err(Ownable2StepError::OwnableInvalidOwner(OwnableInvalidOwner {
                owner: Address::ZERO,
            }));
        }
        self.owner.set(initial_owner);
        Ok(())
    }

    /// Internal function to check if the caller is the owner
    pub fn only_owner(&self) -> Result<(), Ownable2StepError> {
        let caller = self.vm().msg_sender();
        if caller != self.owner.get() {
            return Err(Ownable2StepError::OwnableUnauthorizedAccount(
                OwnableUnauthorizedAccount { account: caller },
            ));
        }
        Ok(())
    }

    /// Internal function to transfer ownership
    fn _transfer_ownership(&mut self, new_owner: Address) {
        let previous_owner = self.owner.get();
        self.owner.set(new_owner);
        self.pending_owner.set(Address::ZERO);
        evm::log(OwnershipTransferred {
            previous_owner,
            new_owner,
        });
    }
}

#[public]
impl Ownable2Step {
    /// Returns the address of the current owner
    pub fn owner(&self) -> Address {
        self.owner.get()
    }

    /// Returns the address of the pending owner
    pub fn pending_owner(&self) -> Address {
        self.pending_owner.get()
    }

    /// Starts the ownership transfer of the contract to a new account
    /// Can only be called by the current owner
    /// Setting newOwner to the zero address can be used to cancel a pending transfer
    pub fn transfer_ownership(&mut self, new_owner: Address) -> Result<(), Ownable2StepError> {
        self.only_owner()?;
        
        let previous_owner = self.owner.get();
        self.pending_owner.set(new_owner);
        
        evm::log(OwnershipTransferStarted {
            previous_owner,
            new_owner,
        });
        
        Ok(())
    }

    /// The new owner accepts the ownership transfer
    pub fn accept_ownership(&mut self) -> Result<(), Ownable2StepError> {
        let sender = self.vm().msg_sender();
        let pending = self.pending_owner.get();
        
        if pending != sender {
            return Err(Ownable2StepError::OwnableUnauthorizedAccount(
                OwnableUnauthorizedAccount { account: sender },
            ));
        }
        
        self._transfer_ownership(sender);
        Ok(())
    }

    /// Leaves the contract without owner. Can only be called by the current owner
    /// NOTE: Renouncing ownership will leave the contract without an owner,
    /// thereby disabling any functionality that is only available to the owner
    pub fn renounce_ownership(&mut self) -> Result<(), Ownable2StepError> {
        self.only_owner()?;
        self._transfer_ownership(Address::ZERO);
        Ok(())
    }
}