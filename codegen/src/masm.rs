//! MacroAssembler used by the code generation.

use crate::{
    abi::Offset,
    asm::{Assembler, BUFFER_LIMIT},
    control::ControlStackFrame,
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
    pub fn patch(&mut self, frame: &ControlStackFrame) -> Result<()> {
        let label = frame.label();
        let pc_offset = (self.pc_offset() as usize)
            .checked_sub(label.len())
            .ok_or(Error::LabelMismatch)?;

        // Patch buffer.
        let buffer = self.asm.buffer();
        let mut new_buffer: SmallVec<[u8; BUFFER_LIMIT]> =
            buffer[..=frame.original_pc_offset as usize].into();
        new_buffer.extend_from_slice(&pc_offset.offset());
        new_buffer.extend_from_slice(
            buffer[(frame.original_pc_offset as usize + 1)..]
                .strip_prefix(label.as_ref())
                .ok_or(Error::LabelMismatch)?,
        );

        *self.asm.buffer_mut() = new_buffer;
        Ok(())
    }

    /// Store data in memory.
    pub fn memory_write(&mut self, ty: impl Offset) -> Result<()> {
        let mem_offset = self.mstore()?.offset();
        self.asm.push(ty.offset().as_ref())?;
        self.asm.push(&mem_offset)?;

        Ok(())
    }

    /// Get the current program counter offset.
    pub fn pc_offset(&self) -> u16 {
        self.asm.buffer().len() as u16
    }
}
