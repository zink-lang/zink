//! Addition example.
#![cfg_attr(target_arch = "wasm32", no_std)]

// for panic handler.
#[cfg(all(target_arch = "wasm32", not(test)))]
extern crate zink;

/// Adds two numbers together.
#[no_mangle]
pub extern "C" fn add(x: u64, y: u64) -> u64 {
    x + y
}
