//! Addition example.
#![no_std]

// for the panic handler.
#[cfg(not(test))]
extern crate zink;

use zink::events::log0;

/// Adds two numbers together.
#[no_mangle]
pub extern "C" fn log(s: &str) {
    let hello = s.as_bytes();

    unsafe {
        log0(hello.as_ptr() as i64, hello.len() as i64);
    }
}
