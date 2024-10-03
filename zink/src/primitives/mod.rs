//! Zink primitive types

mod address;

pub use address::Address;

/// Fixed-sized 20 bytes array
pub(crate) type Bytes24 = [u64; 3];
