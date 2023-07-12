//! MacroAssembler used by the code generation.

use crate::{
    abi::Offset,
    asm::{Assembler, BUFFER_LIMIT},
    control::ControlStackFrame,
    Error, Result,
};
use opcodes::ShangHai as OpCode;
use smallvec::SmallVec;
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
    pub fn patch(&mut self, frame: &ControlStackFrame) -> Result<SmallVec<[u8; 2]>> {
        let frame_offset = frame.pc_offset() as usize;
        // Current buffer offset + JUMPDEST + next-opcode.
        let pc_offset = (self.pc_offset() + 2).offset();

        // Patch buffer.
        let buffer = self.asm.buffer();
        let mut new_buffer: SmallVec<[u8; BUFFER_LIMIT]> = buffer[..frame_offset].into();
        match pc_offset.len() {
            1 => new_buffer.push(OpCode::PUSH1.into()),
            2 => new_buffer.push(OpCode::PUSH2.into()),
            _ => return Err(Error::InvalidPC),
        }
        new_buffer.extend_from_slice(&pc_offset);
        new_buffer.extend_from_slice(&buffer[frame_offset..]);

        *self.asm.buffer_mut() = new_buffer;
        self._jumpdest()?;

        Ok(pc_offset)
    }

    /// Store data in memory.
    pub fn memory_write(&mut self, ty: impl Offset) -> Result<()> {
        let offset = self.memory.len();
        if offset > 32 {
            return Err(Error::MemoryOutOfBounds);
        }

        // use the current memory pointer as offset
        // to store the data.
        let offset = offset.offset();
        self.push(&offset)?;
        self._mstore()?;

        // mock the memory usages.
        let value = ty.offset();
        self.memory.extend_from_slice(value.as_ref());

        // NOTE: post logic for memory write, leave the
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
