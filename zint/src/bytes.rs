//! Utils for bytes conversion.

/// Trait for converting type to bytes32.
pub trait Bytes32: Sized {
    /// Convert type to the lowest significant bytes 32.
    fn to_bytes32(&self) -> [u8; 32];
}

/// Implement Bytes32 for types.
macro_rules! impl_bytes32 {
    ($($ty:ident),+) => {
        $(
            impl Bytes32 for $ty {
                fn to_bytes32(&self) -> [u8; 32] {
                    let mut bytes = [0u8; 32];
                    let le_bytes = self.to_le_bytes();
                    bytes[(32 - le_bytes.len())..].copy_from_slice(&le_bytes);
                    bytes
                }
            }
        )+
    };
}

impl_bytes32!(i8, u8, i16, u16, i32, u32, usize, i64, u64, i128, u128);
