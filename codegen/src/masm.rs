//! MacroAssembler used by the code generation.

use crate::{abi::ToLSBytes, asm::Assembler, control::ControlStackFrame, Result};
use std::ops::{Deref, DerefMut};

/// EVM MacroAssembler.
#[derive(Default)]
pub struct MacroAssembler {
    /// Low level assembler.
    pub asm: Assembler,
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
    pub fn patch(&mut self, frame: &ControlStackFrame) -> Result<usize> {
        let original_pc = frame.pc_offset() as usize;
        let target_pc = self.pc_offset().clone() as usize;
        let buffer = self.asm.buffer_mut();

        crate::patch(buffer, original_pc, target_pc)
    }

    /// Store data in memory.
    pub fn memory_write(&mut self, ty: impl ToLSBytes) -> Result<()> {
        // use the current memory pointer as offset
        // to store the data.
        let offset = self.mp.to_ls_bytes();
        self.push(&offset)?;
        self._mstore()?;

        // mock the memory usages.
        let value = ty.to_ls_bytes();
        self.increment_mp(value.as_ref().len() as u8)?;

        // post logic for memory write, leave the
        // data size and memory offset on the stack.
        self.push(value.as_ref())?; // push value
        self.push(&offset)?; // push offset

        Ok(())
    }

    /// Get the current program counter offset.
    pub fn pc_offset(&self) -> u16 {
        self.asm.buffer().len() as u16
    }
}
