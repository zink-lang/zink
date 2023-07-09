//! MacroAssembler used by the code generation.

use crate::{asm::Assembler, Result};
use smallvec::SmallVec;
use std::ops::{Deref, DerefMut};
use tracing::trace;

/// EVM MacroAssembler.
#[derive(Default)]
pub struct MacroAssembler {
    /// Stack pointer offset.
    ///
    /// NOTE: `u16` is enough since the max stack size is 0x400.
    sp_offset: u16,
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
    /// Increments stack pointer offset.
    pub fn increment_sp(&mut self, offset: u16) {
        self.sp_offset += offset;
    }

    /// Get input data from the current environment.
    pub fn calldata_load(&mut self, value: SmallVec<[u8; 32]>) -> Result<()> {
        trace!("calldata_load: {:x?}", value);

        // NOTE: have offset checks inside the assembler.
        self.asm.push(value.len() as u8)?;
        self.asm.emits(&value);
        self.asm.calldata_load();

        Ok(())
    }
}
