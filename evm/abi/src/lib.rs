//! Solidity ABI implementation
//!
//! https://docs.soliditylang.org/en/latest/abi-spec.html#json
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

mod abi;
mod arg;

#[cfg(not(feature = "std"))]
pub(crate) mod std {
    extern crate alloc;

    pub use alloc::{
        string::{String, ToString},
        vec::Vec,
    };
}

pub use self::{
    abi::Abi,
    arg::{Arg, Param},
};
