//! Memory Instructions

use crate::{Assembler, Result};
use tracing::trace;

impl Assembler {
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
}
