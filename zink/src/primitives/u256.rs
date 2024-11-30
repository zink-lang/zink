#![allow(clippy::should_implement_trait)]
use crate::{ffi, storage::{StorageValue, TransientStorageValue}, Asm};

/// Account address
#[repr(C)]
#[derive(Clone, Copy)]
pub struct U256(
    #[cfg(target_family = "wasm")] i32,
    #[cfg(not(target_family = "wasm"))] [u8; 32],
);

impl U256 {
    /// Returns empty address
    #[cfg(not(target_family = "wasm"))]
    pub const fn empty() -> Self {
        U256([0; 32])
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
}

impl Asm for U256 {
    #[inline(always)]
    fn push(self) {
        unsafe { ffi::asm::push_u256(self) }
    }

    #[cfg(not(target_family = "wasm"))]
    fn bytes32(&self) -> [u8; 32] {
        self.0
    }
}

impl StorageValue for U256 {
    #[inline(always)]
    fn sload() -> Self {
        unsafe { ffi::asm::sload_u256() }
    }
}

impl TransientStorageValue for U256 {
    #[inline(always)]
    fn tload() -> Self {
        unsafe { ffi::asm::tload_u256() }
    }
}