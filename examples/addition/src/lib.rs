//! Addition example.
#![no_std]

// for the panic handler.
#[cfg(not(test))]
extern crate zink;

/// Adds two numbers together.
#[no_mangle]
pub extern "C" fn addition(x: u64, y: u64) -> u64 {
    x + y
}
