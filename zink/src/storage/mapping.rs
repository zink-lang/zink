//! Storage Mapping

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

/// Interface for the key of mappings
pub trait MappingKey {
    // fn storage_key() ->
}
