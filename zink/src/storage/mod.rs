//! Zink storage implementation.

mod mapping;

/// Storage trait. Currently not for public use
pub trait Storage<T> {
    const STORAGE_KEY: i32;

    /// Get value from storage.
    fn get() -> T;

    /// Set value to storage.
    fn set(value: T);
}
