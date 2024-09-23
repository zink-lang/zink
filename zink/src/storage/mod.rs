//! Zink storage implementation.

pub use {
    array::StorageArray,
    kv::{Storage, StorageValue},
    mapping::{MappingKey, StorageMapping},
};

mod array;
mod kv;
mod mapping;
