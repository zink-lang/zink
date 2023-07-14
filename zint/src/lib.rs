//! Zink testing framework.

pub mod bytes;
pub mod evm;
pub mod num;
pub mod utils;

pub use self::{bytes::Bytes32, evm::EVM};
