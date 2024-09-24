//! Storage Array

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
