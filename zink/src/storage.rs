//! Zink storage implementation.

use crate::Asm;

/// Storage trait. Currently not for public use
pub trait Storage<T: Asm> {
    const STORAGE_KEY: i32;

    /// Get value from storage.
    fn get() -> T;

    /// Set value to storage.
    fn set(value: T);
}

/// Storage mapping interface
pub trait StorageMapping<Key, Value> {
    type Index;
    const STORAGE_INDEX: i32;

    /// Get value from storage key.
    fn get(key: &Key) -> Value;

    /// Set key and value
    fn set(key: Key, value: Value) -> Value;

    /// Size of this mapping
    fn size() -> Self::Index;

    /// If key exists
    fn exists(key: Key) -> bool;
}

/// Storage array interface
pub trait StorageArray<Value> {
    type Index;
    const STORAGE_INDEX: i32;

    /// Get value from storage key.
    fn get(index: &Self::Index) -> Value;

    /// Set value to index
    fn set(index: &Self::Index, value: Value);

    /// Set value to index
    fn push(value: Value);

    /// Size of array
    fn size() -> Self::Index;
}
