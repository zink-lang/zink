//! WASM local slot.

use crate::abi::Type;
use wasmparser::ValType;

/// A local slot.
///
/// Represents the type, location and addressing mode of a local
/// in the stack's local and argument area.
pub struct LocalSlot {
    /// The type contained by this local slot.
    inner: ValType,

    /// The offset of this local slot.
    _offset: usize,
}

impl LocalSlot {
    /// Create a new local slot.
    pub fn new(offset: usize, inner: ValType) -> Self {
        Self {
            _offset: offset,
            inner,
        }
    }
}

impl Type for LocalSlot {
    fn size(&self) -> usize {
        self.inner.size()
    }
}
