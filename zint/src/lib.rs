//! Zink testing framework.
#![deny(missing_docs)]

pub mod bytes;
pub mod evm;

pub use self::{bytes::Bytes32, evm::EVM};
