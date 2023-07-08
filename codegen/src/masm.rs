//! MacroAssembler used by the code generation.

use crate::{asm::Assembler, limits::StackOffset, Error, Result};
use std::ops::{Deref, DerefMut};
use tracing::trace;

/// EVM MacroAssembler.
#[derive(Default)]
pub struct MacroAssembler {
    /// Stack pointer offset.
    sp_offset: StackOffset,
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
    /// Buffer of the assembler.
    pub fn buffer(&self) -> &[u8] {
        self.asm.buffer()
    }

    /// Increments stack pointer offset.
    pub fn increment_sp(&mut self, offset: impl Into<StackOffset>) {
        self.sp_offset += offset.into();
    }

    /// Get input data of current environment
    ///
    /// convert the u32 index to u8 array for the
    /// stack representation of EVM.
    ///
    /// NOTE:
    ///
    /// per stack item of evm is 32 bytes.
    pub fn calldata_load(&mut self, index: u32) -> Result<()> {
        let offset = if index != 0 {
            index
                .checked_mul(32)
                .ok_or(Error::LocalIndexOutOfRange)?
                .to_le_bytes()
                .iter()
                .rev()
                .skip_while(|x| **x == 0)
                .copied()
                .collect::<Vec<_>>()
                .iter()
                .rev()
                .copied()
                .collect::<Vec<_>>()
                .to_vec()
        } else {
            vec![0] // PUSH1 0x00
        };

        trace!("calldata_load: {:x?}", offset);
        self.asm.push(
            offset
                .len()
                .try_into()
                .map_err(|_| Error::StackIndexOutOfRange)?,
        )?;
        self.asm.emits(&offset)?;
        self.asm.calldata_load();

        Ok(())
    }
}
