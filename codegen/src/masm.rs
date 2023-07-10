//! MacroAssembler used by the code generation.

use crate::{
    asm::{Assembler, BUFFER_LIMIT},
    Error, Result, Stack,
};
use smallvec::SmallVec;
use std::ops::{Deref, DerefMut};

/// EVM MacroAssembler.
#[derive(Default)]
pub struct MacroAssembler {
    /// Low level assembler.
    pub asm: Assembler,

    /// Virtual stack for compilation.
    pub stack: Stack,
}

impl Deref for MacroAssembler {
    type Target = Assembler;

    fn deref(&self) -> &Self::Target {
        &self.asm
    }
}

impl DerefMut for MacroAssembler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asm
    }
}

impl MacroAssembler {
    /// Patch label with the current program counter.
    pub fn patch(&mut self, offset: u16, label: [u8; 3]) -> Result<()> {
        let offset = offset as usize;
        let pc_offset = self.pc_offset() as usize - 3;
        let buffer = self.asm.buffer();

        let mut new_buffer: SmallVec<[u8; BUFFER_LIMIT]> = buffer[..offset].into();
        new_buffer.extend_from_slice(&pc_offset.to_le_bytes());
        new_buffer.extend_from_slice(
            buffer[offset..]
                .strip_prefix(&label)
                .ok_or_else(|| Error::LabelMismatch)?,
        );

        *self.asm.buffer_mut() = new_buffer;
        Ok(())
    }

    /// Get the program counter offset.
    pub fn pc_offset(&self) -> u16 {
        self.asm.buffer().len() as u16
    }
}
