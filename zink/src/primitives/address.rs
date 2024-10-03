use crate::{ffi, primitives::Bytes24, storage::StorageValue, Asm};

/// Account address
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Address(Bytes24);

impl PartialEq for Address {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::address_eq(*self, *other) }
    }
}

impl Eq for Address {}

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
