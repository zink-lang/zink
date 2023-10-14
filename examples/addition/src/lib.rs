//! Addition example.
#![no_std]

// for the panic handler.
#[cfg(not(test))]
extern crate zink;

use zink::external;

/// Adds two numbers together.
#[external]
pub fn addition(x: u64, y: u64) -> u64 {
    x + y
}
