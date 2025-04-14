//! Zink storage implementation.

use crate::{isa, Asm};
pub use {
    dkmapping::{DoubleKeyMapping, DoubleKeyTransientMapping},
    mapping::{Mapping, TransientMapping},
    value::{Storage, TransientStorage},
};

pub mod dkmapping;
pub mod mapping;
mod value;

/// Interface for the value of kv based storage
pub trait StorageValue: Asm {
    /// Load from storage
    fn sload() -> Self;
}

/// Interface for the value of kv based transient storage
pub trait TransientStorageValue: Asm {
    /// Load from transient storage
    fn tload() -> Self;
}

impl StorageValue for i32 {
    fn sload() -> Self {
        unsafe { isa::asm::sload_i32() }
    }
}

impl StorageValue for u32 {
    fn sload() -> Self {
        unsafe { isa::asm::sload_u32() }
    }
}

impl TransientStorageValue for i32 {
    fn tload() -> Self {
        unsafe { isa::asm::tload_i32() }
    }
}

impl TransientStorageValue for u32 {
    fn tload() -> Self {
        unsafe { isa::asm::tload_u32() }
    }
}
