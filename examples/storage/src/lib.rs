//! Addition example.
#![no_std]

// for the panic handler.
#[cfg(not(test))]
extern crate zink;

static mut COUNTER: i32 = 0;

/// Set value to storage and get it
#[no_mangle]
pub unsafe extern "C" fn set_and_get(x: i32) -> i32 {
    COUNTER = x;
    COUNTER
}

/// Set value to storage
#[no_mangle]
pub unsafe extern "C" fn set(x: i32) {
    COUNTER = x;
}

/// Get value from storage
#[no_mangle]
pub unsafe extern "C" fn get() -> i32 {
    COUNTER
}
