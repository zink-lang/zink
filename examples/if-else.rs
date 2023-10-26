//! if-else example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

/// Simple if-else condition
#[zink::external]
pub fn if_else(x: u64, y: u64) -> u64 {
    if x > y {
        x
    } else {
        y
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
