//! Memory Instructions

use crate::{MacroAssembler, Result};
use tracing::trace;

impl MacroAssembler {
    /// Load n bytes to extend self as another number type.
    ///
    /// Just for adapting the WASM instructions, this method makes
    /// no sense for EVM since all of the numbers as U256.
    pub(crate) fn _load(&mut self) -> Result<()> {
        trace!("load");
        Ok(())
    }

    /// Load 1 byte to extend self as another number type.
    ///
    /// Just for adapting the WASM instructions, this method makes
    /// no sense for EVM since all of the numbers as U256.
    pub(crate) fn _load8(&mut self) -> Result<()> {
        trace!("load8");
        Ok(())
    }

    /// Load 2 bytes to extend self as another number type.
    ///
    /// Just for adapting the WASM instructions, this method makes
    /// no sense for EVM since all of the numbers as U256.
    pub(crate) fn _load16(&mut self) -> Result<()> {
        trace!("load16");
        Ok(())
    }

    /// Load 4 bytes to extend self as another number type.
    ///
    /// Just for adapting the WASM instructions, this method makes
    /// no sense for EVM since all of the numbers as U256.
    pub(crate) fn _load32(&mut self) -> Result<()> {
        trace!("load32");
        Ok(())
    }

    /// Store n bytes in memory.
    pub fn _store(&mut self) -> Result<()> {
        todo!()
    }

    /// Wrap self to i8 and store 1 byte
    pub fn _store8(&mut self) -> Result<()> {
        todo!()
    }

    /// Wrap self to i16 and store 2 bytes
    pub fn _store16(&mut self) -> Result<()> {
        todo!()
    }

    /// Wrap self to i32 and store 4 bytes
    pub fn _store32(&mut self) -> Result<()> {
        todo!()
    }

    /// The memory size instruction returns the current
    /// size of memory.
    pub fn _memory_size(&mut self, _: u32, _: u8) -> Result<()> {
        todo!()
    }

    /// The memory grow instruction grows memory by a given
    /// delta and returns the previous size, or -1 if enough
    /// memory cannot be allocated.
    pub fn _memory_grow(&mut self, _: u32, _: u8) -> Result<()> {
        todo!()
    }
}
