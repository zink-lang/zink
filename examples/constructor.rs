//! Storage example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::Storage;

/// It gets expanded to 'Counter' struct
/// that implements zink::Storage trait
/// (::set and ::get)
///
/// Storage key is taken based on macro order
/// (e.g this macro is first and only in this project,
/// so it will take 0x0 contract storage key)
#[zink::storage]
pub type Counter = i32;

/// Get value from the storage.
#[zink::external]
pub fn get() -> i32 {
    Counter::get()
}

/// Set value to the storage.
#[zink::constructor]
pub fn constructor(value: i32) {
    Counter::set(value);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
