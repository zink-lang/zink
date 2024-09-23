//! Key-Value storage
use crate::{ffi, Asm};

/// Storage trait. Currently not for public use
pub trait Storage<T: Asm> {
    const STORAGE_KEY: i32;

    /// Get value from storage.
    fn get() -> T;

    /// Set value to storage.
    fn set(value: T) {
        Asm::push(value);
        Asm::push(Self::STORAGE_KEY);
        unsafe {
            ffi::evm::sstore();
        }
    }
}
