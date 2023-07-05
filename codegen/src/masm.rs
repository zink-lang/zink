//! MacroAssembler used by the code generation.

use crate::{asm::Assmbler, limits::StackOffset};

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
    pub fn increment_sp(&mut self, offset: impl Into<StackOffset>) {
        self.sp_offset += offset.into();
    }

    /// Add instruction combinations.
    pub fn add(&mut self, lhs: u8, rhs: u8) {
        self.asm.push::<2>();
        self.increment_sp(2);
        self.asm.emits([lhs, rhs]);
        self.asm.add();
    }
}
