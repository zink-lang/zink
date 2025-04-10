#![allow(clippy::should_implement_trait)]

use crate::{
    ffi,
    primitives::Bytes32,
    storage::{StorageValue, TransientStorageValue},
    Asm,
};
use core::ops::Sub;

/// Account address
#[repr(C)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct U256(Bytes32);

impl U256 {
    /// Returns empty value  
    pub const fn empty() -> Self {
        U256(Bytes32::empty())
    }

    /// u256 add
    #[inline(always)]
    pub fn add(self, other: Self) -> Self {
        #[cfg(target_arch = "wasm32")]
        unsafe {
            ffi::u256_add(self, other)
        }

        #[cfg(not(target_arch = "wasm32"))]
        ffi::u256_add(self, other)
    }

    /// u256 less than
    #[inline(always)]
    pub fn lt(self, other: Self) -> bool {
        #[cfg(target_arch = "wasm32")]
        unsafe {
            ffi::u256_lt(other, self)
        }

        #[cfg(not(target_arch = "wasm32"))]
        ffi::u256_lt(other, self)
    }

    /// u256 eq
    #[inline(always)]
    pub fn eq(self, other: Self) -> bool {
        self.0.eq(other.0)
    }

    /// u256 sub
    #[inline(always)]
    pub fn sub(self, other: Self) -> Self {
        #[cfg(target_arch = "wasm32")]
        unsafe {
            ffi::u256_sub(other, self)
        }

        #[cfg(not(target_arch = "wasm32"))]
        ffi::u256_sub(other, self)
    }

    /// u256 div
    #[inline(always)]
    pub fn div(self, other: Self) -> Self {
        unsafe { ffi::u256_div(self, other) }
    }

    /// max of u256
    #[inline(always)]
    pub fn max() -> Self {
        #[cfg(target_arch = "wasm32")]
        unsafe {
            ffi::u256_max()
        }

        #[cfg(not(target_arch = "wasm32"))]
        ffi::u256_max()
    }

    pub fn to_bytes32(&self) -> Bytes32 {
        self.0
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn bytes32(&self) -> [u8; 32] {
        self.0 .0 // [u8; 32] in non-WASM
    }

    #[inline(always)]
    pub fn addmod(self, other: Self, modulus: Self) -> Self {
        #[cfg(target_arch = "wasm32")]
        unsafe {
            ffi::u256_addmod(modulus, other, self)
        }

        #[cfg(not(target_arch = "wasm32"))]
        ffi::u256_addmod(modulus, other, self)
    }

    /// Mulmod for U256
    #[inline(always)]
    pub fn mulmod(self, other: Self, modulus: Self) -> Self {
        #[cfg(target_arch = "wasm32")]
        unsafe {
            ffi::u256_mulmod(modulus, other, self)
        }

        #[cfg(not(target_arch = "wasm32"))]
        ffi::u256_mulmod(modulus, other, self)
    }
}

impl Sub for U256 {
    type Output = Self;

    /// u256 sub
    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        #[cfg(target_arch = "wasm32")]
        unsafe {
            ffi::u256_sub(self, other)
        }

        #[cfg(not(target_arch = "wasm32"))]
        ffi::u256_sub(self, other)
    }
}

impl Asm for U256 {
    #[inline(always)]
    fn push(self) {
        #[cfg(target_arch = "wasm32")]
        unsafe {
            ffi::bytes::push_bytes32(self.0)
        }

        #[cfg(not(target_arch = "wasm32"))]
        ffi::bytes::push_bytes32(self.0)
    }

    #[cfg(not(target_family = "wasm"))]
    fn bytes32(&self) -> [u8; 32] {
        self.bytes32() // Delegate to the instance method
    }
}

impl StorageValue for U256 {
    #[inline(always)]
    fn sload() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            Self(unsafe { ffi::bytes::sload_bytes32() })
        }

        #[cfg(not(target_arch = "wasm32"))]
        Self(ffi::bytes::sload_bytes32())
    }
}

impl TransientStorageValue for U256 {
    #[inline(always)]
    fn tload() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            Self(unsafe { ffi::bytes::tload_bytes32() })
        }

        #[cfg(not(target_arch = "wasm32"))]
        Self(ffi::bytes::tload_bytes32())
    }
}

impl From<u64> for U256 {
    fn from(value: u64) -> Self {
        #[cfg(target_family = "wasm")]
        {
            U256(Bytes32(value as i32))
        }
        #[cfg(not(target_family = "wasm"))]
        {
            // On non-WASM, Bytes32 is [u8; 32]
            let mut bytes = [0u8; 32];
            bytes[24..32].copy_from_slice(&value.to_be_bytes());
            U256(Bytes32(bytes))
        }
    }
}
