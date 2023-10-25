//! Storage example.
#![cfg_attr(target_arch = "wasm32", no_std)]

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

/// Set value to the storage and get it.
#[zink::external]
pub fn set_and_get(value: i32) -> i32 {
    Counter::set(value);
    Counter::get()
}

/// set value to the storage.
#[zink::external]
pub fn set(value: i32) {
    Counter::set(value);
}

/// Get value from the storage.
#[zink::external]
pub fn get() -> i32 {
    Counter::get()
}

fn main() {}
