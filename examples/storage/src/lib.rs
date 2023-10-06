//! Addition example.
#![no_std]

// for the panic handler.
#[cfg(not(test))]
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
#[no_mangle]
pub unsafe extern "C" fn set_and_get(value: i32) -> i32 {
    Counter::set(value);
    Counter::get()
}

/// Set value to the storage.
#[no_mangle]
pub unsafe extern "C" fn set(value: i32) {
    Counter::set(value);
}

/// Get value from the storage.
#[no_mangle]
pub unsafe extern "C" fn get() -> i32 {
    Counter::get()
}
