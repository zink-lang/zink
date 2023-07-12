//! Compilation buffers

use crate::Result;
use zingen::{Buffer as CBuffer, Labels};

/// The output buffer of codegen
pub struct Buffer {
    inner: CBuffer,
    labels: Labels,
}

impl Buffer {
    /// New compilation buffer.
    pub fn new(buffer: CBuffer, labels: Labels) -> Self {
        Self {
            inner: buffer,
            labels,
        }
    }

    /// Get the generated buffer.
    pub fn buffer(&self) -> &[u8] {
        &self.inner
    }

    /// Get the mutable generated buffer.
    pub fn buffer_mut(&mut self) -> &mut [u8] {
        &mut self.inner
    }

    /// If buffer is completed.
    pub fn completed(&self) -> bool {
        self.labels.is_empty()
    }

    /// Patch function call to this buffer.
    pub fn patch(&mut self, _index: u32, _offset: u16) -> Result<()> {
        // zingen::patch(&mut self.inner, index, offset)
        Ok(())
    }
}

impl From<(CBuffer, Labels)> for Buffer {
    fn from((buffer, labels): (CBuffer, Labels)) -> Self {
        Self::new(buffer, labels)
    }
}
