//! WASM local slot.

use crate::abi::Type;
use smallvec::{smallvec, SmallVec};
use wasmparser::ValType;

/// A local slot.
///
/// Represents the type, location and addressing mode of a local
/// in the stack's local and argument area.
pub struct LocalSlot {
    /// The type contained by this local slot.
    inner: ValType,

    /// The offset of this local slot.
    ///
    /// TODO: make this offset to u256. (#20)
    offset: usize,
}

impl LocalSlot {
    /// Create a new local slot.
    pub fn new(offset: usize, inner: ValType) -> Self {
        Self { offset, inner }
    }

    /// Get the offset of this local slot in the
    /// lowest significant bytes.
    pub fn index(&self) -> SmallVec<[u8; 256]> {
        if self.offset == 0 {
            return smallvec![0];
        }

        self.offset
            .to_le_bytes()
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

impl Type for LocalSlot {
    fn size(&self) -> usize {
        self.inner.size()
    }
}
