#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::{primitives::Address, Event};

/// ERC20 standard events
#[derive(Event)]
pub enum ERC20Events {
    /// Emitted when tokens are transferred between addresses
    /// Parameters: from, to, value
    Transfer(Address, Address, u64),

    /// Emitted when an address approves another address to spend tokens
    /// Parameters: owner, spender, value
    Approval(Address, Address, u64),
}

// /// Implementation of event logging functions
// pub trait ERC20EventLogger {
//     fn log_transfer(&self, from: Address, to: Address, value: U256) ;
//     fn log_approval(&self, owner: Address, spender: Address, value: U256) ;
// }

// impl ERC20EventLogger for ERC20Events {
//     fn log_transfer(&self, from: Address, to: Address, value: U256) {
//         let event = ERC20Events::Transfer(from, to, value);
//         let topic = event.topic();
//         let data = event.encode();
//         event.log1(&topic);
//     }

//     fn log_approval(&self, owner: Address, spender: Address, value: U256) {
//         let event = ERC20Events::Approval(owner, spender, value);
//         let topic = event.topic();
//         let data = event.encode();
//         event.log1(&topic);
//     }
// }

// /// Example contract functions demonstrating event logging
// #[zink::external]
// pub mod erc20_events {
//     use super::*;

//     /// Logs a transfer event
//     pub fn log_transfer(from: Address, to: Address, value: U256) {
//         let events = ERC20Events::Transfer(from, to, value);
//         events.log_transfer(from, to, value);
//     }

//     /// Logs an approval event
//     pub fn log_approval(owner: Address, spender: Address, value: U256)  {
//         let events = ERC20Events::Approval(owner, spender, value);
//         events.log_approval(owner, spender, value);
//     }

//     /// Example of logging multiple events in a single transaction
//     pub fn log_transfer_and_approval(
//         from: Address,
//         to: Address,
//         spender: Address,
//         value: U256,
//     ) {
//         // Log transfer first
//         log_transfer(from, to, value)?;
//         // Then log approval
//         log_approval(from, spender, value)?;
//         Ok(())
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use zink::test_utils::{Address as TestAddress, U256 as TestU256};

//     #[test]
//     fn test_event_signatures() {
//         let transfer = ERC20Events::Transfer(
//             TestAddress::zero(),
//             TestAddress::zero(),
//             TestU256::from(0),
//         );
//         let approval = ERC20Events::Approval(
//             TestAddress::zero(),
//             TestAddress::zero(),
//             TestU256::from(0),
//         );

//         assert_eq!(
//             transfer.abi_signature(),
//             "Transfer(address,address,uint256)"
//         );
//         assert_eq!(
//             approval.abi_signature(),
//             "Approval(address,address,uint256)"
//         );
//     }

//     #[test]
//     fn test_event_logging() -> Result<(), EventError> {
//         let from = TestAddress::from([1u8; 20]);
//         let to = TestAddress::from([2u8; 20]);
//         let spender = TestAddress::from([3u8; 20]);
//         let value = TestU256::from(1000);

//         // Test individual event logging
//         erc20_events::log_transfer(from, to, value)?;
//         erc20_events::log_approval(from, spender, value)?;

//         // Test multiple events
//         erc20_events::log_transfer_and_approval(from, to, spender, value)?;

//         Ok(())
//     }
// }

// // Only include main when not targeting wasm32
#[cfg(not(target_arch = "wasm32"))]
fn main() {}
