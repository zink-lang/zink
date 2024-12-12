#![allow(clippy::should_implement_trait)]

use crate::{
    ffi,
    primitives::Bytes32,
    storage::{StorageValue, TransientStorageValue},
    Asm,
};

/// Account address
#[repr(C)]
#[derive(Clone, Copy)]
pub struct U256(Bytes32);

impl U256 {
    /// Returns empty value  
    pub const fn empty() -> Self {
        U256(Bytes32::empty())
    }

    /// u256 add
    #[inline(always)]
    pub fn add(self, other: Self) -> Self {
        unsafe { ffi::u256_add(self, other) }
    }

    /// u256 less than
    #[inline(always)]
    pub fn lt(self, other: Self) -> bool {
        unsafe { ffi::u256_lt(other, self) }
    }

    /// u256 eq
    #[inline(always)]
    pub fn eq(self, other: Self) -> bool {
        self.0.eq(other.0)
    }

    /// u256 sub
    #[inline(always)]
    pub fn sub(self, other: Self) -> Self {
        unsafe { ffi::u256_sub(other, self) }
    }

    /// max of u256
    #[inline(always)]
    pub fn max() -> Self {
        unsafe { ffi::u256_max() }
    }

    /// U256 to bytes32
    pub fn bytes32(&self) -> Bytes32 {
        self.0
    }

    /// Addmod for U256
    #[inline(always)]
    pub fn addmod(self, other: Self, modulus: Self) -> Self {
        unsafe { ffi::u256_addmod(modulus, other, self) }
    }

    /// Mulmod for U256
    #[inline(always)]
    pub fn mulmod(self, other: Self, modulus: Self) -> Self {
        unsafe { ffi::u256_mulmod(modulus, other, self) }
    }
}

impl Asm for U256 {
    #[inline(always)]
    fn push(self) {
        unsafe { ffi::bytes::push_bytes32(self.0) }
    }

    #[cfg(not(target_family = "wasm"))]
    fn bytes32(&self) -> [u8; 32] {
        self.0 .0
    }
}

impl StorageValue for U256 {
    #[inline(always)]
    fn sload() -> Self {
        Self(unsafe { ffi::bytes::sload_bytes32() })
    }
}

impl TransientStorageValue for U256 {
    #[inline(always)]
    fn tload() -> Self {
        Self(unsafe { ffi::bytes::tload_bytes32() })
    }
}
