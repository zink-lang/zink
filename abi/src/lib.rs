//! Zink ABI implementation
//!
//! Currently just a wrapper of solidity ABI.

mod abi;
pub mod result;
pub mod selector;

pub use abi::Abi;

#[cfg(feature = "selector")]
pub use selector::keccak256;
