//! Zink storage implementation.

pub use {array::StorageArray, kv::Storage, mapping::StorageMapping};

mod array;
mod kv;
mod mapping;
