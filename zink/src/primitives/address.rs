use crate::{ffi, storage::StorageValue, Asm};

/// Account address
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Address(i32);

impl Address {
    /// if self equal to another
    pub fn eq(self, other: Self) -> bool {
        unsafe { ffi::address_eq(self, other) }
    }
}

impl Asm for Address {
    fn push(self) {
        unsafe { ffi::asm::push_address(self) }
    }
}

impl StorageValue for Address {
    fn sload() -> Self {
        unsafe { ffi::asm::sload_address() }
    }
}
