//! Key-Value storage
use crate::{ffi, storage::StorageValue, Asm};

/// Storage trait. Currently not for public use
pub trait Storage {
    const STORAGE_KEY: i32;
    type Value: StorageValue + Asm;

    /// Get value from storage.
    fn get() -> Self::Value {
        Asm::push(Self::STORAGE_KEY);
        Self::Value::sload()
    }

    /// Set value to storage.
    fn set(value: Self::Value) {
        value.push();
        Asm::push(Self::STORAGE_KEY);
        unsafe {
            ffi::evm::sstore();
        }
    }
}
