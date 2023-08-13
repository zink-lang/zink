//! Addition example.
#![no_std]

// for the panic handler.
#[cfg(not(test))]
extern crate zink;

use zink::storage::{sload, sstore};

/// TODO: generate this storage interface with proc macro.
struct Counter;

impl Counter {
    fn get() -> i64 {
        unsafe { sload(0) }
    }

    fn set(value: i64) {
        unsafe { sstore(0, value) }
    }
}

/// Set value to storage and get it
#[no_mangle]
pub unsafe extern "C" fn set_and_get(value: i64) -> i64 {
    Counter::set(value);
    Counter::get()
}

/// Set value to storage
#[no_mangle]
pub unsafe extern "C" fn set(value: i64) {
    Counter::set(value);
}

/// Get value from storage
#[no_mangle]
pub unsafe extern "C" fn get() -> i64 {
    Counter::get()
}
