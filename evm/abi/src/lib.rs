//! Solidity ABI implementation
//!
//! https://docs.soliditylang.org/en/latest/abi-spec.html#json
#![deny(missing_docs)]

mod abi;
mod input;

pub use self::{
    abi::Abi,
    input::{Input, Param},
};
