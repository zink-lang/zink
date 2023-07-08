//! WASM ABI

use std::ops::{Add, BitAnd, Not, Sub};
use wasmparser::ValType;

/// A local slot.
///
/// Represents the type, location and addressing mode of a local
/// in the stack's local and argument area.
pub struct LocalSlot {
    /// The offset of the local slot.
    pub offset: u32,
    /// The type contained by this local slot.
    pub ty: ValType,
}

impl LocalSlot {
    /// Create a new local slot.
    pub fn new(ty: ValType, offset: u32) -> Self {
        Self { offset, ty }
    }

    /// Get the size of the local slot.
    pub fn size(&self) -> u32 {
        let bytes = match self.ty {
            ValType::I32 | ValType::F32 => 4,
            ValType::I64 | ValType::F64 => 8,
            // TODO: align number implementations to 256 bits (issue #20)
            _ => unimplemented!("unknown unsupported type {:?}", self.ty),
        };

        // rounding up to the nearest 32 bytes
        let mut floor = bytes / 32;
        if bytes % 32 != 0 {
            floor += 1;
        }

        floor
    }
}

/// Align a value up to the given power-of-two-alignment.
// See https://sites.google.com/site/theoryofoperatingsystems/labs/malloc/align8
pub(crate) fn align_to<N>(value: N, alignment: N) -> N
where
    N: Not<Output = N>
        + BitAnd<N, Output = N>
        + Add<N, Output = N>
        + Sub<N, Output = N>
        + From<u8>
        + Copy,
{
    let alignment_mask = alignment - 1.into();
    (value + alignment_mask) & !alignment_mask
}
