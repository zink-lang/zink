//! Zink primitive types

mod address;
mod bytes;
mod u256;

pub use {address::Address, bytes::*, u256::U256};

pub type Bytes20 = Address;
pub type Bytes32 = U256;
pub type String32 = U256;
