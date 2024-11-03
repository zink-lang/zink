//! Utils for bytes conversion.
//!
//! TODO: move this util to other library

/// Trait for converting type to bytes32.
pub trait Bytes32: Sized {
    /// Convert type to the lowest significant bytes 32.
    fn to_bytes32(&self) -> [u8; 32];

    /// Convert type to vec of bytes.
    fn to_vec(&self) -> Vec<u8> {
        self.to_bytes32().to_vec()
    }
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

                fn to_vec(&self) -> Vec<u8> {
                    self.to_le_bytes().to_vec()
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

    fn to_vec(&self) -> Vec<u8> {
        self.clone()
    }
}

impl Bytes32 for [u8; 20] {
    fn to_bytes32(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        bytes[12..].copy_from_slice(self);
        bytes
    }
}

impl Bytes32 for [u8; 32] {
    fn to_bytes32(&self) -> [u8; 32] {
        *self
    }

    fn to_vec(&self) -> Vec<u8> {
        self.as_ref().into()
    }
}

impl Bytes32 for &[u8] {
    fn to_bytes32(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        bytes[(32 - self.len())..].copy_from_slice(self);
        bytes
    }

    fn to_vec(&self) -> Vec<u8> {
        (*self).into()
    }
}

impl Bytes32 for () {
    fn to_bytes32(&self) -> [u8; 32] {
        [0; 32]
    }

    fn to_vec(&self) -> Vec<u8> {
        Default::default()
    }
}

impl Bytes32 for &str {
    fn to_bytes32(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        bytes[(32 - self.len())..].copy_from_slice(self.as_bytes());
        bytes
    }

    fn to_vec(&self) -> Vec<u8> {
        self.as_bytes().into()
    }
}

impl Bytes32 for bool {
    fn to_bytes32(&self) -> [u8; 32] {
        let mut output = [0; 32];
        if *self {
            output[31] = 1;
        }

        output
    }
}

impl_bytes32!(i8, u8, i16, u16, i32, u32, usize, i64, u64, i128, u128);
