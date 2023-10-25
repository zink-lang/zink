//! Addition example.
#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate zink;

/// Adds two numbers together.
#[zink::external]
pub fn addition(x: u64, y: u64) -> u64 {
    x + y
}

// This is required when because we want to
// build this with cargo b.
fn main() {}
