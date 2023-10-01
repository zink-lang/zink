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
                    let ls_bytes = {
                        self.to_le_bytes()
                            .into_iter()
                            .rev()
                            .skip_while(|b| *b == 0)
                            .collect::<Vec<_>>()
                            .into_iter()
                            .rev()
                            .collect::<Vec<_>>()
                    };

                    bytes[(32 - ls_bytes.len())..].copy_from_slice(&ls_bytes);
                    bytes
                }
            }
        )+
    };
}

impl Bytes32 for Vec<u8> {
    fn to_bytes32(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        bytes[(32 - self.len())..].copy_from_slice(self);
        bytes
    }
}

impl Bytes32 for &[u8] {
    fn to_bytes32(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        bytes[(32 - self.len())..].copy_from_slice(self);
        bytes
    }
}

impl_bytes32!(i8, u8, i16, u16, i32, u32, usize, i64, u64, i128, u128);
