//! Double key mapping

use crate::{asm, storage::Value};

/// Storage mapping interface
pub trait DoubleKeyMapping {
    const STORAGE_SLOT: i32;

    type Key1: Value;
    type Key2: Value;
    type Value: Value;

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
        unsafe {
            asm::evm::sstore();
        }
    }
}

/// Transient storage mapping interface
pub trait DoubleKeyTransientMapping {
    const STORAGE_SLOT: i32;

    type Key1: Value;
    type Key2: Value;
    type Value: Value;

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
        unsafe {
            asm::evm::tstore();
        }
    }
}

/// Load storage key to stack
#[inline(always)]
pub fn load_double_key(key1: impl Value, key2: impl Value, index: i32) {
    unsafe {
        asm::label_reserve_mem_64();

        // write key1 to memory
        key1.push();
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

        // stores the hash
        asm::evm::push0();
        asm::evm::mstore();

        // write index to memory
        key2.push();
        asm::ext::push_u8(0x20);
        asm::evm::mstore();

        // hash key
        asm::ext::push_u8(0x40);
        asm::evm::push0();
        asm::evm::keccak256();
    }
}
