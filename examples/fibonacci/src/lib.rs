//! fibonacci example.
#![no_std]

// for the panic handler.
#[cfg(not(test))]
extern crate zink;

/// Calculates the nth fibonacci number.
#[no_mangle]
pub extern "C" fn recursion(n: usize) -> usize {
    if n == 0 || n == 1 {
        n
    } else {
        recursion(n - 1) + recursion(n - 2)
    }
}
