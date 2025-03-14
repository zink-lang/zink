//! Zink ABI implementation
//!
//! Currently just a wrapper of solidity ABI.

mod abi;
#[cfg(feature = "encoding")]
mod encoding;
pub mod result;
pub mod selector;
#[cfg(feature = "encoding")]
pub use encoding::{decode, encode, is_dynamic_type, AbiDecode, AbiEncode, DecodeError};

pub use abi::Abi;

#[cfg(feature = "selector")]
pub use selector::keccak256;
