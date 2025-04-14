//! Key-Value storage
use crate::{asm, storage::Value};

/// Storage trait. Currently not for public use
pub trait Storage {
    #[cfg(not(target_family = "wasm"))]
    const STORAGE_KEY: [u8; 32];
    const STORAGE_SLOT: i32;

    type Value: Value;

    /// Get value from storage.
    fn get() -> Self::Value {
        Value::push(Self::STORAGE_SLOT);
        Self::Value::sload()
    }

    /// Set value to storage.
    fn set(value: Self::Value) {
        value.push();
        Value::push(Self::STORAGE_SLOT);
        unsafe {
            asm::evm::sstore();
        }
    }
}

/// Transient storage trait. Currently not for public use
pub trait TransientStorage {
    #[cfg(not(target_family = "wasm"))]
    const STORAGE_KEY: [u8; 32];
    const STORAGE_SLOT: i32;

    type Value: Value;

    /// Get value from transient storage.
    fn get() -> Self::Value {
        Value::push(Self::STORAGE_SLOT);
        Self::Value::tload()
    }

    /// Set value to transient storage.
    fn set(value: Self::Value) {
        value.push();
        Value::push(Self::STORAGE_SLOT);
        unsafe {
            asm::evm::tstore();
        }
    }
}
