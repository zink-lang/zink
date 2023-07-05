//! MacroAssembler used by the code generation.

use crate::{asm::Assmbler, stack::StackOffset};

/// EVM MacroAssembler.
#[derive(Default)]
pub struct MacroAssembler {
    /// Stack pointer offset.
    sp_offset: StackOffset,
    /// Low level assembler.
    asm: Assmbler,
}

impl MacroAssembler {
    /// Buffer of the assembler.
    pub fn buffer(&self) -> &[u8] {
        self.asm.buffer()
    }

    /// Increments stack pointer offset.
    pub fn push(&mut self, offset: impl Into<StackOffset>) {
        self.sp_offset += offset.into();
    }
}
