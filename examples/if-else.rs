//! if-else example.
#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate zink;

/// Simple if-else condition
#[no_mangle]
pub extern "C" fn if_else(x: u64, y: u64) -> u64 {
    if x > y {
        x
    } else {
        y
    }
}

fn main() {}
