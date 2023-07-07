//! MacroAssembler used by the code generation.

use crate::{asm::Assmbler, limits::StackOffset};

/// EVM MacroAssembler.
#[derive(Default)]
pub struct MacroAssembler {
    /// Stack pointer offset.
    sp_offset: StackOffset,
    /// Low level assembler.
    pub asm: Assmbler,
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

    /// Get input data of current environment
    pub fn calldata_load(&mut self, offset: u8) {
        // FIXME:
        //
        // PUSH <SIZE> from offset.
        self.asm.push::<1>();
        self.asm.emit(offset);
        self.asm.calldata_load();
    }
}
