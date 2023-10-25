//! Zink testing framework.
#![deny(missing_docs)]

mod bytes;
mod contract;
mod evm;

pub use self::{
    bytes::Bytes32,
    contract::Contract,
    evm::{Info, InstructionResult, EVM, U256},
};
