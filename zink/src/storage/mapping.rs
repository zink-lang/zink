//! Storage Mapping

use crate::{ffi, storage::StorageValue, Asm};

/// Storage mapping interface
pub trait Mapping {
    const STORAGE_SLOT: i32;

    type Key: MappingKey;
    type Value: StorageValue;

    /// Get value from storage key.
    fn get(key: &Self::Key) -> Self::Value {
        key.load(Self::STORAGE_SLOT);
        Self::Value::sload()
    }

    /// Set key and value
    fn set(key: Self::Key, value: Self::Value) {
        value.push();
        key.load(Self::STORAGE_SLOT);
        unsafe {
            ffi::evm::sstore();
        }
    }
}

/// Interface for the key of mappings
pub trait MappingKey: Asm {
    const SIZE: i32;

    /// Load storage key to stack
    fn load(self, index: i32) {
        unsafe {
            // write index to memory
            index.push();
            ffi::evm::push0();
            ffi::evm::mstore8();

            // write key to memory
            self.push();
            ffi::evm::push1(0x01);
            ffi::evm::mstore();

            // hash key
            Self::SIZE.push();
            ffi::evm::push0();
            ffi::evm::keccak256();
        }
    }
}

macro_rules! impl_mapping_key {
    (($ty:ident, $size:expr)) => {
        impl MappingKey for $ty {
            const SIZE: i32 = $size;
        }
    };
    ($len:expr) => {
        impl MappingKey for [u8; $len] {
            const SIZE: i32 = $len;
        }
    };
    ($($ty:tt),+) => {
        $(impl_mapping_key!($ty);)+
    };
}

impl_mapping_key! {
    (i8, 1), (u8, 1),
    (i16, 2), (u16, 2),
    (i32, 4), (u32, 4),
    (i64, 4), (u64, 4),
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
}
