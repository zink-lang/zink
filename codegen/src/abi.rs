//! WASM ABI

use wasmparser::ValType;

/// The alignment mask for 32 bytes (32 - 1).
const ALIGNMENT_MASK: usize = 31;

/// WASM type size for the stack representation of EVM.
pub trait Type {
    /// Alignment the size of this type to 32 bytes for the
    /// stack representation of EVM.
    ///
    /// `u16` here is enough since the maximum stack size of
    /// EVM is 0x400 bytes.
    ///
    /// See https://sites.google.com/site/theoryofoperatingsystems/labs/malloc/align8
    fn align(&self) -> usize {
        (self.size() + ALIGNMENT_MASK) & !ALIGNMENT_MASK
    }

    /// Size in bytes.
    fn size(&self) -> usize;
}

impl Type for ValType {
    fn size(&self) -> usize {
        match self {
            ValType::I32 | ValType::F32 => 4,
            ValType::I64 | ValType::F64 => 8,
            // TODO: align number implementations to 256 bits (issue #20)
            _ => unimplemented!("unknown unsupported type {self}"),
        }
    }
}
