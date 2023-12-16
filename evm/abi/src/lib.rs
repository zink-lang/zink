//! Solidity ABI implementation
//!
//! https://docs.soliditylang.org/en/latest/abi-spec.html#json
#![deny(missing_docs)]

mod abi;
mod arg;

pub use self::{
    abi::Abi,
    arg::{Arg, Param},
};
