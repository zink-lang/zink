//! Zink primitive types

mod address;
mod u256;

pub use address::{Address, Caller};
pub use u256::U256;

pub type Bytes20 = Address;
pub type Bytes32 = U256;
pub type String32 = U256;
