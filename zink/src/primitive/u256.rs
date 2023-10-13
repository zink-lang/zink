//! 256 bit unsigned integer

/// 256 bit unsigned integer
pub struct U256([u64; 4]);

impl From<u64> for U256 {
    fn from(val: u64) -> Self {
        U256([val, 0, 0, 0])
    }
}

impl From<u32> for U256 {
    fn from(val: u32) -> Self {
        U256([val as u64, 0, 0, 0])
    }
}
