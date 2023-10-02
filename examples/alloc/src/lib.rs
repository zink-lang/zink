//! Allocator example.
#![no_std]

// for the panic handler.
#[cfg(not(test))]
extern crate zink;

use zink::U256;

const PING: &[u8] = b"ping";

#[no_mangle]
pub extern "C" fn alloc(b: i32) {
    let num = U256::try_from(PING.as_ref()).unwrap();
}
