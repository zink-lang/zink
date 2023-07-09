//! WASM ABI

use smallvec::{smallvec, SmallVec};
use wasmparser::ValType;

/// The alignment mask for 32 bytes (32 - 1).
const ALIGNMENT_MASK: usize = 31;

// The maximum size of a custom number in EVM is `u256`
// which still takes 32 bytes.
const EVM_NUMBER_SIZE: u8 = 32;

/// WASM type size for the stack representation of EVM.
pub trait Type {
    /// Alignment the size of this type to 32 bytes for the
    /// stack representation of EVM.
    ///
    /// See https://sites.google.com/site/theoryofoperatingsystems/labs/malloc/align8
    fn align(&self) -> usize {
        (self.size() + ALIGNMENT_MASK) & !ALIGNMENT_MASK
    }

    /// If the value is number.
    fn is_number(&self) -> bool;

    /// Size in bytes.
    fn size(&self) -> usize;

    /// The Value that is about to be pushed on the stack.
    fn value(&self) -> SmallVec<[u8; 32]>;
}

impl Type for ValType {
    fn is_number(&self) -> bool {
        match self {
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 => true,
            _ => unimplemented!("unknown unsupported type {self}"),
        }
    }

    fn size(&self) -> usize {
        match self {
            ValType::I32 | ValType::F32 => 4,
            ValType::I64 | ValType::F64 => 8,
            // TODO: align number implementations to 256 bits (issue #20)
            _ => unimplemented!("unknown unsupported type {self}"),
        }
    }

    fn value(&self) -> SmallVec<[u8; 32]> {
        match self {
            // The maximum size of a custom number in EVM is `u256`
            // which still takes 32 bytes.
            n if n.is_number() => smallvec![EVM_NUMBER_SIZE],
            _ => unimplemented!("unknown unsupported type {self}"),
        }
    }
}
