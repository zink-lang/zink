//! Allocator example.
#![no_std]

// for the panic handler.
#[cfg(not(test))]
extern crate zink;

#[no_mangle]
pub extern "C" fn alloc(b: i32) -> u8 {
    let mut bytes = [0u8; 4];
    let r = b.to_le_bytes();
    bytes.copy_from_slice(&r);

    bytes[0]
}
