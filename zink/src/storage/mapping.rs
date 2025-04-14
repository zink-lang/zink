//! Storage Mapping

use crate::{asm, storage::Value};

/// Storage mapping interface
pub trait Mapping {
    const STORAGE_SLOT: i32;

    type Key: Value;
    type Value: Value;

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
            asm::evm::sstore();
        }
    }
}

/// Transient storage mapping interface
pub trait TransientMapping {
    const STORAGE_SLOT: i32;

    type Key: Value;
    type Value: Value;

    #[cfg(not(target_family = "wasm"))]
    fn storage_key(key: Self::Key) -> [u8; 32];

    /// Get value from transient storage key.
    fn get(key: Self::Key) -> Self::Value {
        load_key(key, Self::STORAGE_SLOT);
        Self::Value::tload()
    }

    /// Set key and value in transient storage
    fn set(key: Self::Key, value: Self::Value) {
        value.push();
        load_key(key, Self::STORAGE_SLOT);

        unsafe {
            asm::evm::tstore();
        }
    }
}

/// Load storage key to stack
pub fn load_key(key: impl Value, index: i32) {
    unsafe {
        asm::label_reserve_mem_32();

        // write key to memory
        key.push();
        asm::evm::push0();
        asm::evm::mstore();

        // write index to memory
        index.push();
        asm::ext::push_u8(0x20);
        asm::evm::mstore();

        // hash key
        asm::ext::push_u8(0x40);
        asm::evm::push0();
        asm::evm::keccak256();
    }
}
