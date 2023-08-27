//! Addition example.
#![no_std]

// for the panic handler.
#[cfg(not(test))]
extern crate zink;

/// TODO: generate this storage interface with proc macro.
use zink::ffi::evm::{sload, sstore};

// The number `0` in this struct is for the storage key,
// it will be convreted to `0x000..0000`.
struct Counter;

impl Counter {
    fn get() -> i64 {
        unsafe { sload(0) }
    }

    fn set(value: i64) {
        unsafe { sstore(0, value) }
    }
}

/// Set value to the storage and get it.
#[no_mangle]
pub unsafe extern "C" fn set_and_get(value: i64) -> i64 {
    Counter::set(value);
    Counter::get()
}

/// Set value to the storage.
#[no_mangle]
pub unsafe extern "C" fn set(value: i64) {
    Counter::set(value);
}

/// Get value from the storage.
#[no_mangle]
pub unsafe extern "C" fn get() -> i64 {
    Counter::get()
}
