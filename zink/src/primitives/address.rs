use crate::{ffi, storage::StorageValue, Asm};

/// Account address
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Address(
    #[cfg(target_family = "wasm")] i32,
    #[cfg(not(target_family = "wasm"))] [u8; 20],
);

impl Address {
    /// Returns empty address
    #[cfg(not(target_family = "wasm"))]
    pub const fn empty() -> Self {
        Address([0; 20])
    }

    /// if self equal to another
    ///
    /// NOTE: not using core::cmp because it uses registers in wasm
    #[allow(clippy::should_implement_trait)]
    pub fn eq(self, other: Self) -> bool {
        unsafe { ffi::address_eq(self, other) }
    }
}

impl Asm for Address {
    fn push(self) {
        unsafe { ffi::asm::push_address(self) }
    }

    #[cfg(not(target_family = "wasm"))]
    fn bytes32(&self) -> [u8; 32] {
        let mut output = [0; 32];
        output[12..].copy_from_slice(&self.0);
        output
    }
}

impl StorageValue for Address {
    fn sload() -> Self {
        unsafe { ffi::asm::sload_address() }
    }
}
