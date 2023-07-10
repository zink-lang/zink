//! if-else example.
#![cfg_attr(target_arch = "wasm32", no_std)]

// for panic handler.
#[cfg(all(target_arch = "wasm32", not(test)))]
extern crate zink;

/// Simple if-else condition
#[no_mangle]
pub extern "C" fn main(x: u64, y: u64) -> u64 {
    if x > y {
        x
    } else {
        y
    }
}
