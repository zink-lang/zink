//! Zink primitive types

mod address;
pub mod bytes;
pub mod numeric;
mod u256;

pub use {address::Address, bytes::*, u256::U256};

// pub type Address = Bytes20;
// pub type Bytes32 = U256;
pub type String32 = U256;
