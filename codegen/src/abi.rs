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
}

// /// A stack argument.
// pub struct Argument {}
//
// /// A function signature.
// pub struct Signature {}

/// Returns the size in bytes of a given WebAssembly type.
pub(crate) fn ty_size(ty: &ValType) -> u32 {
    match *ty {
        ValType::I32 | ValType::F32 => 4,
        ValType::I64 | ValType::F64 => 8,
        // TODO: support u128, u256
        _ => unimplemented!("unknown unsupported type {:?}", ty),
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
