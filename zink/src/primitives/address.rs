use crate::{ffi, storage::StorageValue, Asm};
use super::Bytes32;

/// Account address
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Address(
    #[cfg(target_family = "wasm")] i32,
    #[cfg(not(target_family = "wasm"))] pub [u8; 20],
);

impl Address {
    /// Returns empty address
    #[cfg(not(target_family = "wasm"))]
    pub const fn empty() -> Self {
        Address([0; 20])
    }

    /// Returns empty address
    #[cfg(target_family = "wasm")]
    pub const fn empty() -> Self {
        Address(0)
    }

    /// Returns empty address
    #[inline(always)]
    pub fn caller() -> Self {
        unsafe { ffi::evm::caller() }
    }

    /// if self equal to another
    ///
    /// NOTE: not using core::cmp because it uses registers in wasm
    #[allow(clippy::should_implement_trait)]
    #[inline(always)]
    pub fn eq(self, other: Self) -> bool {
        unsafe { ffi::address_eq(self, other) }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn to_bytes32(&self) -> Bytes32 {        
        let mut output = [0; 32];
        output[12..].copy_from_slice(&self.0);
        // Bytes32::from(U256::from(output))
        Bytes32::empty()
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
