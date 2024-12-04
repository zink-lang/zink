#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::{
    primitives::{Address, U256},
    Asm, Event,
};

/// ERC20 standard events
#[derive(Event)]
pub enum ERC20Events {
    /// Emitted when tokens are transferred between addresses
    /// Parameters: from, to, value
    Transfer(Address, Address, U256),

    /// Emitted when an address approves another address to spend tokens
    /// Parameters: owner, spender, value
    Approval(Address, Address, U256),
}

impl ERC20Events {
    /// Log a transfer event
    pub fn log_transfer(from: Address, to: Address, value: U256) {
        match Self::Transfer(from, to, value) {
            event => event.log0(),
        }
    }

    /// Log an approval event
    pub fn log_approval(owner: Address, spender: Address, value: U256) {
        match Self::Approval(owner, spender, value) {
            event => event.log0(),
        }
    }
}

pub mod erc20_events {
    use super::*;

    /// Logs a transfer event
    /// Example contract functions demonstrating event logging
    #[zink::external]
    pub fn log_transfer(from: Address, to: Address, value: U256) {
        ERC20Events::log_transfer(from, to, value)
    }

    /// Logs an approval event
    /// Example contract functions demonstrating event logging
    #[zink::external]
    pub fn log_approval(owner: Address, spender: Address, value: U256) {
        ERC20Events::log_approval(owner, spender, value)
    }

    /// Example of logging multiple events in a single transaction
    /// Example contract functions demonstrating event logging
    #[zink::external]
    pub fn log_transfer_and_approval(
        from: Address,
        to: Address,
        spender: Address,
        value: U256,
    ) -> Result<(), ()> {
        // Log transfer first
        log_transfer(from, to, value);
        // Then log approval
        log_approval(from, spender, value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zink::EventError;

    #[test]
    fn test_event_logging() -> Result<(), EventError> {
        let from = Address([1u8; 20]);
        let to = Address([2u8; 20]);
        let spender = Address([3u8; 20]);
        let value = U256::from(1000u64);

        // Test individual event logging
        erc20_events::log_transfer(from, to, value)?;
        erc20_events::log_approval(from, spender, value)?;

        // Test multiple events
        erc20_events::log_transfer_and_approval(from, to, spender, value)?;

        Ok(())
    }
}

// Only include main when not targeting wasm32
#[cfg(not(target_arch = "wasm32"))]
fn main() {}
