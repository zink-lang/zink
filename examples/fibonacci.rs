//! fibonacci example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

// for the panic handler.
extern crate zink;

/// Calculates the nth fibonacci number.
#[zink::external]
pub fn fibonacci(n: usize) -> usize {
    recursion(n)
}

/// Calculates the nth fibonacci number using recursion.
#[zink::external]
pub fn recursion(n: usize) -> usize {
    if n < 2 {
        n
    } else {
        recursion(n - 1) + recursion(n - 2)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
