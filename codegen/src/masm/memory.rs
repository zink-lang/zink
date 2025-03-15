//! Memory Instructions

use crate::{MacroAssembler, Result};

impl MacroAssembler {
    /// Load n bytes to extend self as another number type.
    ///
    /// Just for adapting the WASM instructions, this method makes
    /// no sense for EVM since all of the numbers as U256.
    pub(crate) fn _load(&mut self) -> Result<()> {
        Ok(())
    }

    /// Load 1 byte to extend self as another number type.
    ///
    /// Just for adapting the WASM instructions, this method makes
    /// no sense for EVM since all of the numbers as U256.
    pub(crate) fn _load8(&mut self) -> Result<()> {
        Ok(())
    }

    /// Load 2 bytes to extend self as another number type.
    ///
    /// Just for adapting the WASM instructions, this method makes
    /// no sense for EVM since all of the numbers as U256.
    pub(crate) fn _load16(&mut self) -> Result<()> {
        Ok(())
    }

    /// Load 4 bytes to extend self as another number type.
    ///
    /// Just for adapting the WASM instructions, this method makes
    /// no sense for EVM since all of the numbers as U256.
    pub(crate) fn _load32(&mut self) -> Result<()> {
        Ok(())
    }

    /// Store n bytes in memory.
    pub fn _store(&mut self) -> Result<()> {
        tracing::warn!("_store is a placeholder implementation for EVM compatibility");

        // EVM stack: [value, offset]
        // MSTORE expects [offset, value], so swap the top two items
        self._swap1()?;
        // Stack: [offset, value]

        // Store the value using MSTORE (32 bytes, EVM standard)
        self._mstore()?;

        Ok(())
    }

    /// Wrap self to i8 and store 1 byte
    pub fn _store8(&mut self) -> Result<()> {
        tracing::warn!("_store8 is a placeholder implementation for EVM compatibility");

        // EVM stack: [value, offset]
        // Duplicate the top two items to preserve them
        self._dup2()?;

        // The stack is now [value, offset, value, offset]
        // Pop only the duplicated value to balance
        self._pop()?; // Remove the duplicated value
                      // Stack: [value, offset, offset]

        // The top item is the offset; use it for MSTORE
        // EVM MSTORE expects [value, offset], so swap to get [offset, value]
        self._swap1()?;
        // Stack: [offset, value]

        // Store the value (MSTORE will take the 32-byte word, but we want only 1 byte)
        self._mstore()?;

        Ok(())
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
