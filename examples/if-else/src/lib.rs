//! if-else example.
#![no_std]

// for panic handler.
#[cfg(not(test))]
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
