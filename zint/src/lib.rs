//! Zink testing framework.
#![deny(missing_docs)]

mod bytes;
mod evm;

pub use self::{
    bytes::Bytes32,
    evm::{Info, InstructionResult, EVM, U256},
};
