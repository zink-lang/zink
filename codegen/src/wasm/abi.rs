//! WASM ABI

use smallvec::{smallvec, SmallVec};
use wasmparser::{BlockType, ValType};

/// The alignment mask for 32 bytes (32 - 1).
const ALIGNMENT_MASK: usize = 31;

/// WASM type size for the stack representation of EVM.
pub trait Type {
    /// Alignment the size of this type to bytes32 for the
    /// stack representation of EVM.
    ///
    /// See <https://sites.google.com/site/theoryofoperatingsystems/labs/malloc/align8>
    fn align(&self) -> usize {
        (self.size() + ALIGNMENT_MASK) & !ALIGNMENT_MASK
    }

    /// Size in bytes.
    fn size(&self) -> usize;
}

impl Type for &ValType {
    fn size(&self) -> usize {
        match self {
            ValType::I32 | ValType::F32 => 4,
            ValType::I64 | ValType::F64 => 8,
            // TODO: align number implementations to 256 bits (issue #20)
            _ => unimplemented!("unknown unsupported type {self}"),
        }
    }
}

impl Type for ValType {
    fn size(&self) -> usize {
        (&self).size()
    }
}

impl Type for &[ValType] {
    fn size(&self) -> usize {
        self.as_ref().iter().map(|t| t.align()).sum::<usize>()
    }
}

impl Type for usize {
    fn align(&self) -> usize {
        (*self + ALIGNMENT_MASK) & !ALIGNMENT_MASK
    }

    fn size(&self) -> usize {
        self.to_le_bytes().len()
    }
}

/// Get the offset of this type in the lowest
/// significant bytes.
pub trait ToLSBytes {
    /// Output buffer
    type Output: AsRef<[u8]>;

    /// Convert self to the lowest significant bytes.
    fn to_ls_bytes(&self) -> Self::Output;
}

macro_rules! offset {
    ($number:ident, $size:expr) => {
        impl ToLSBytes for $number {
            type Output = SmallVec<[u8; $size]>;

            fn to_ls_bytes(&self) -> Self::Output {
                if *self == 0 {
                    return smallvec![0];
                }

                self.to_le_bytes()
                    .into_iter()
                    .rev()
                    .skip_while(|b| *b == 0)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>()
                    .into()
            }
        }
    };
    ($(($number:ident, $size:expr)),+) => {
        $(offset!($number, $size);)+
    };
}

offset! {
    (usize, 8),
    (u64, 8),
    (i64, 8),
    (i32, 4),
    (u32, 4),
    (u16, 2),
    (u8, 1)
}

impl ToLSBytes for BlockType {
    type Output = SmallVec<[u8; 8]>;

    fn to_ls_bytes(&self) -> Self::Output {
        match self {
            BlockType::Empty => Default::default(),
            BlockType::Type(val) => val.size().to_ls_bytes(),
            BlockType::FuncType(val) => Self::Output::from_slice(&val.to_ls_bytes()),
        }
    }
}

impl ToLSBytes for &ValType {
    type Output = SmallVec<[u8; 8]>;

    fn to_ls_bytes(&self) -> Self::Output {
        self.align().to_ls_bytes()
    }
}

impl ToLSBytes for &[ValType] {
    type Output = SmallVec<[u8; 8]>;

    fn to_ls_bytes(&self) -> Self::Output {
        self.as_ref()
            .iter()
            .map(|t| t.align())
            .sum::<usize>()
            .to_ls_bytes()
    }
}
