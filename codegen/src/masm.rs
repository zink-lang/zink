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
    /// Emit PUSH operation
    fn emit_push(&mut self) -> Result<()> {
        let address = self.stack.address();
        self.asm.push(address.len() as u8)?;
        self.asm.data(&address);

        Ok(())
    }

    /// Push a byte on the stack, increments the stack pointer.
    pub fn push(&mut self, byte: u8) -> Result<()> {
        self.emit_push()?;
        self.stack.push(byte)?;
        Ok(())
    }

    /// Push bytes on the stack, increments the stack pointer.
    pub fn pushn<const S: usize>(&mut self, bytes: [u8; S]) -> Result<()> {
        self.emit_push()?;
        self.stack.pushn(bytes)?;
        Ok(())
    }

    /// Patch label with the current program counter.
    pub fn patch(&mut self, offset: u16, label: [u8; 3]) -> Result<()> {
        let offset = offset as usize;
        let pc_offset = self.pc_offset() as usize - 3;
        let buffer = self.asm.buffer();

        let mut new_buffer: SmallVec<[u8; BUFFER_LIMIT]> = buffer[..offset].into();
        new_buffer.extend_from_slice(&pc_offset.to_le_bytes());
        new_buffer.extend_from_slice(
            &buffer[offset..]
                .strip_prefix(&label)
                .ok_or_else(|| Error::LabelMismatch)?
                .to_vec(),
        );

        *self.asm.buffer_mut() = new_buffer;
        Ok(())
    }

    /// Get the program counter offset.
    pub fn pc_offset(&self) -> u16 {
        self.asm.buffer().len() as u16
    }
}
