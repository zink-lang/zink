//! Zink storage implementation.

use crate::{ffi, Asm};
pub use {dkmapping::DoubleKeyMapping, mapping::Mapping, value::Storage};

mod dkmapping;
mod mapping;
mod value;

/// Interface for the value of kv based storage
pub trait StorageValue: Asm {
    /// Load from storage
    fn sload() -> Self;
}

impl StorageValue for i32 {
    fn sload() -> Self {
        unsafe { ffi::asm::sload_i32() }
    }
}
