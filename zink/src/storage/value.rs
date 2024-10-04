//! Key-Value storage
use crate::{ffi, storage::StorageValue, Asm};

/// Storage trait. Currently not for public use
pub trait Storage {
    #[cfg(not(target_family = "wasm"))]
    const KEY: [u8; 32];
    const STORAGE_SLOT: i32;

    type Value: StorageValue + Asm;

    /// Get value from storage.
    fn get() -> Self::Value {
        Asm::push(Self::STORAGE_SLOT);
        Self::Value::sload()
    }

    /// Set value to storage.
    fn set(value: Self::Value) {
        value.push();
        Asm::push(Self::STORAGE_SLOT);
        unsafe {
            ffi::evm::sstore();
        }
    }
}
