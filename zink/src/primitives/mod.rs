//! Zink primitive types

pub mod address;
pub mod bytes;
pub mod numeric;
pub mod u256;

pub use {address::Address, bytes::*, u256::U256};

// pub type Address = Bytes20;
// pub type Bytes32 = U256;
pub type String32 = U256;
