use crate::{
    storage::{StorageValue, TransientStorageValue},
    Asm,
};

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub use native::{load_key, sstore, tstore};
#[cfg(target_arch = "wasm32")]
pub use wasm::{load_key, sstore, tstore};

pub trait Mapping {
    const STORAGE_SLOT: i32;
    type Key: Asm;
    type Value: StorageValue;

    #[cfg(not(target_family = "wasm"))]
    fn storage_key(key: Self::Key) -> [u8; 32];

    fn get(key: Self::Key) -> Self::Value {
        load_key(key, Self::STORAGE_SLOT);
        Self::Value::sload()
    }

    fn set(key: Self::Key, value: Self::Value) {
        value.push();
        load_key(key, Self::STORAGE_SLOT);
        sstore();
    }
}

pub trait TransientMapping {
    const STORAGE_SLOT: i32;
    type Key: Asm;
    type Value: TransientStorageValue;

    #[cfg(not(target_family = "wasm"))]
    fn storage_key(key: Self::Key) -> [u8; 32];

    fn get(key: Self::Key) -> Self::Value {
        load_key(key, Self::STORAGE_SLOT);
        Self::Value::tload()
    }

    fn set(key: Self::Key, value: Self::Value) {
        value.push();
        load_key(key, Self::STORAGE_SLOT);
        tstore();
    }
}
