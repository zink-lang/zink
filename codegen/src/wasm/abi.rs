//! WASM ABI
//!
//! This module provides functionality for defining the WASM ABI, including type sizing and
//! alignment for stack representation compatible with the EVM.

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

/// Custom type for Ethereum address (20 bytes)
#[derive(Clone, Copy)]
pub struct Address(pub [u8; 20]);

impl Type for Address {
    fn size(&self) -> usize {
        20 // 20 bytes for an Ethereum address
    }

    fn align(&self) -> usize {
        // No need to align to 32 bytes for memory passing (optional)
        self.size()
    }
}

/// Custom type for fixed arrays
#[derive(Clone)]
pub struct FixedArray<T: Type> {
    data: Vec<T>,
    #[allow(dead_code)]
    len: usize,
}

impl<T: Type> FixedArray<T> {
    /// Creates a new `FixedArray` from a vector of elements.
    pub fn new(data: Vec<T>) -> Self {
        let len = data.len();
        Self { data, len }
    }
}

impl<T: Type> Type for FixedArray<T> {
    fn size(&self) -> usize {
        self.data.iter().map(|item| item.size()).sum::<usize>()
    }

    fn align(&self) -> usize {
        // Keep as actual size for memory efficiency (override 32-byte alignment)
        self.size()
    }
}

/// Get the offset of this type in the lowest significant bytes.
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

impl ToLSBytes for Address {
    type Output = SmallVec<[u8; 20]>;

    fn to_ls_bytes(&self) -> Self::Output {
        smallvec::SmallVec::from_slice(&self.0)
    }
}

impl<T: Type + ToLSBytes> ToLSBytes for FixedArray<T> {
    type Output = SmallVec<[u8; 64]>;

    fn to_ls_bytes(&self) -> Self::Output {
        let mut bytes = SmallVec::new();
        // Uncomment if length prefix is required (per `clearloop`’s ambiguity)
        // bytes.extend_from_slice(&(self.len as u32).to_le_bytes());
        for item in &self.data {
            bytes.extend_from_slice(item.to_ls_bytes().as_ref());
        }
        bytes
    }
}

#[test]
fn test_usize_to_ls_bytes() {
    assert_eq!(363usize.to_ls_bytes().to_vec(), vec![0x01, 0x6b]);
    assert_eq!(255usize.to_ls_bytes().to_vec(), vec![0xff]);
}

#[test]
fn test_address_to_ls_bytes() {
    let addr = Address([1u8; 20]);
    assert_eq!(addr.to_ls_bytes().to_vec(), vec![1u8; 20]);
}

#[test]
fn test_fixed_array_to_ls_bytes() {
    let array = FixedArray::new(vec![
        Address([1u8; 20]),
        Address([2u8; 20]),
        Address([3u8; 20]),
    ]);
    let bytes = array.to_ls_bytes();
    assert_eq!(bytes.len(), 60); // 3 * 20
    assert_eq!(&bytes[0..20], &[1u8; 20]);
    assert_eq!(&bytes[20..40], &[2u8; 20]);
    assert_eq!(&bytes[40..60], &[3u8; 20]);
}
