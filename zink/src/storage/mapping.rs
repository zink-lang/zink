//! Storage Mapping

use crate::{ffi, storage::StorageValue, Asm};

/// Storage mapping interface
pub trait Mapping {
    const STORAGE_SLOT: i32;

    type Key: Asm;
    type Value: StorageValue;

    #[cfg(not(target_family = "wasm"))]
    fn storage_key(key: Self::Key) -> [u8; 32];

    /// Get value from storage key.
    fn get(key: Self::Key) -> Self::Value {
        load_key(key, Self::STORAGE_SLOT);
        Self::Value::sload()
    }

    /// Set key and value
    fn set(key: Self::Key, value: Self::Value) {
        value.push();
        load_key(key, Self::STORAGE_SLOT);
        unsafe {
            ffi::evm::sstore();
        }
    }
}

/// Load storage key to stack
fn load_key(key: impl Asm, index: i32) {
    unsafe {
        // write key to memory
        key.push();
        ffi::evm::push0();
        ffi::evm::mstore();

        // write index to memory
        index.push();
        ffi::asm::push_u8(0x20);
        ffi::evm::mstore();

        // hash key
        ffi::asm::push_u8(0x40);
        ffi::evm::push0();
        ffi::evm::keccak256();
    }
}
