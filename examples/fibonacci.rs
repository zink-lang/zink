//! fibonacci example.
#![cfg_attr(target_arch = "wasm32", no_std)]

// for the panic handler.
extern crate zink;

/// Calculates the nth fibonacci number.
#[no_mangle]
pub extern "C" fn fibonacci(n: usize) -> usize {
    recursion(n)
}

/// Calculates the nth fibonacci number using recursion.
#[no_mangle]
pub extern "C" fn recursion(n: usize) -> usize {
    if n < 2 {
        n
    } else {
        recursion(n - 1) + recursion(n - 2)
    }
}

fn main() {}
