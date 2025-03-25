//! Double key mapping

use crate::{
    storage::{StorageValue, TransientStorageValue},
    Asm,
};

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub use native::{load_double_key, sstore, tstore};
#[cfg(target_arch = "wasm32")]
pub use wasm::{load_double_key, sstore, tstore};

/// Storage mapping interface
pub trait DoubleKeyMapping {
    const STORAGE_SLOT: i32;
    type Key1: Asm;
    type Key2: Asm;
    type Value: StorageValue;

    #[cfg(not(target_family = "wasm"))]
    fn storage_key(key1: Self::Key1, key2: Self::Key2) -> [u8; 32];

    /// Get value from storage key.
    #[inline(always)]
    fn get(key1: Self::Key1, key2: Self::Key2) -> Self::Value {
        load_double_key(key1, key2, Self::STORAGE_SLOT);
        Self::Value::sload()
    }

    /// Set key and value
    #[inline(always)]
    fn set(key1: Self::Key1, key2: Self::Key2, value: Self::Value) {
        value.push();
        load_double_key(key1, key2, Self::STORAGE_SLOT);
        sstore();
    }
}

/// Transient storage mapping interface
pub trait DoubleKeyTransientMapping {
    const STORAGE_SLOT: i32;
    type Key1: Asm;
    type Key2: Asm;
    type Value: TransientStorageValue;

    #[cfg(not(target_family = "wasm"))]
    fn storage_key(key1: Self::Key1, key2: Self::Key2) -> [u8; 32];

    /// Get value from transient storage key.
    #[inline(always)]
    fn get(key1: Self::Key1, key2: Self::Key2) -> Self::Value {
        load_double_key(key1, key2, Self::STORAGE_SLOT);
        Self::Value::tload()
    }

    /// Set key and value in transient storage
    #[inline(always)]
    fn set(key1: Self::Key1, key2: Self::Key2, value: Self::Value) {
        value.push();
        load_double_key(key1, key2, Self::STORAGE_SLOT);
        tstore();
    }
}
