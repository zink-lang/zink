//! Key-Value storage
use crate::{
    ffi,
    storage::{StorageValue, TransientStorageValue},
    Asm,
};

/// Storage trait. Currently not for public use
pub trait Storage {
    #[cfg(not(target_family = "wasm"))]
    const STORAGE_KEY: [u8; 32];
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
        #[cfg(target_arch = "wasm32")]
        unsafe {
            ffi::evm::sstore();
        }
        #[cfg(not(target_arch = "wasm32"))]
        ffi::evm::sstore();
    }
}

/// Transient storage trait. Currently not for public use
pub trait TransientStorage {
    #[cfg(not(target_family = "wasm"))]
    const STORAGE_KEY: [u8; 32];
    const STORAGE_SLOT: i32;

    type Value: TransientStorageValue + Asm;

    /// Get value from transient storage.
    fn get() -> Self::Value {
        Asm::push(Self::STORAGE_SLOT);
        Self::Value::tload()
    }

    /// Set value to transient storage.
    fn set(value: Self::Value) {
        value.push();
        Asm::push(Self::STORAGE_SLOT);
        #[cfg(target_arch = "wasm32")]
        unsafe {
            ffi::evm::tstore();
        }
        #[cfg(not(target_arch = "wasm32"))]
        ffi::evm::tstore();
    }
}
