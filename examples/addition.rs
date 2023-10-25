//! Addition example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]
extern crate zink;

/// Adds two numbers together.
#[zink::external]
pub fn addition(x: u64, y: u64) -> u64 {
    x + y
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
