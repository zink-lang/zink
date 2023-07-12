//! WASM local slot.

use crate::abi::{ToLSBytes, Type};
use smallvec::SmallVec;
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
    pub fn to_ls_bytes(&self) -> SmallVec<[u8; 8]> {
        self.offset.to_ls_bytes()
    }
}

impl Type for LocalSlot {
    fn size(&self) -> usize {
        self.inner.size()
    }
}
