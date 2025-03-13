use crate::{
    ffi,
    primitives::{Bytes20, Bytes32},
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

    // Return Bytes32 for consistency with logN
    pub fn to_bytes32(&self) -> Bytes32 {
        #[cfg(target_family = "wasm")]
        {
            Bytes32(self.0 .0) // i32 in WASM
        }
        #[cfg(not(target_family = "wasm"))]
        {
            let mut bytes = [0u8; 32];
            bytes[12..32].copy_from_slice(&self.0 .0); // [u8; 20] padded
            Bytes32(bytes)
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn bytes32(&self) -> [u8; 32] {
        self.to_bytes32().0 // Use to_bytes32 for non-WASM
    }
}

impl Asm for Address {
    fn push(self) {
        unsafe { ffi::bytes::push_bytes20(self.0) }
    }

    #[cfg(not(target_family = "wasm"))]
    fn bytes32(&self) -> [u8; 32] {
        self.bytes32()
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
