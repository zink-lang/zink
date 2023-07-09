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
    offset: usize,
}

impl LocalSlot {
    /// Create a new local slot.
    pub fn new(offset: usize, inner: ValType) -> Self {
        Self { offset, inner }
    }
}

impl Type for LocalSlot {
    fn is_number(&self) -> bool {
        self.inner.is_number()
    }

    fn size(&self) -> usize {
        self.inner.size()
    }

    fn value(&self) -> SmallVec<[u8; 32]> {
        if self.offset < u8::MAX as usize {
            return smallvec![self.offset as u8];
        }

        let mut value = smallvec![];
        let mut reminder = self.offset;
        while reminder > u8::MAX as usize {
            value.push(u8::MAX);
            reminder -= u8::MAX as usize;
        }

        value.push(reminder as u8);
        tracing::trace!("local slot offest: {}, value: {:?}", self.offset, value);
        value
    }
}
