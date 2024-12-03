use crate::{
    ffi,
    primitives::Bytes20,
    storage::{StorageValue, TransientStorageValue},
    Asm,
};

/// Account address
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Address(Bytes20);

impl Address {
    /// Returns empty address
    pub const fn empty() -> Self {
        Address(Bytes20::empty())
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
        self.0.eq(other.0)
    }
}

impl Asm for Address {
    fn push(self) {
        unsafe { ffi::bytes::push_bytes20(self.0) }
    }

    #[cfg(not(target_family = "wasm"))]
    fn bytes32(&self) -> [u8; 32] {
        self.0.bytes32()
    }
}

impl StorageValue for Address {
    fn sload() -> Self {
        Self(unsafe { ffi::bytes::sload_bytes20() })
    }
}

impl From<Bytes20> for Address {
    fn from(value: Bytes20) -> Self {
        Address(value)
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<[u8; 20]> for Address {
    fn from(value: [u8; 20]) -> Self {
        Address(Bytes20(value))
    }
}

impl TransientStorageValue for Address {
    fn tload() -> Self {
        Address(unsafe { ffi::bytes::tload_bytes20() })
    }
}
