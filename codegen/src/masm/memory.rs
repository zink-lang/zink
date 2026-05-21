//! Memory Instructions

use crate::{wasm::ToLSBytes, MacroAssembler, Result};
use wasmparser::MemArg;

impl MacroAssembler {
    /// Load n bytes to extend self as another number type.
    ///
    /// Just for adapting the WASM instructions, this method makes
    /// no sense for EVM since all of the numbers as U256.
    pub(crate) fn _load(&mut self, arg: MemArg) -> Result<()> {
        self._load_bytes(arg, 4)
    }

    /// Load 8 bytes.
    pub(crate) fn _load64(&mut self, arg: MemArg) -> Result<()> {
        self._load_bytes(arg, 8)
    }

    fn _load_bytes(&mut self, arg: MemArg, bytes: usize) -> Result<()> {
        if arg.offset > 0 {
            self.push(&arg.offset.to_ls_bytes())?;
            self._add()?;
        }

        self._mload()?;
        self.mask_low_bytes(bytes)
    }

    fn mask_low_bytes(&mut self, bytes: usize) -> Result<()> {
        if bytes >= 32 {
            return Ok(());
        }

        self.push(&vec![0xff; bytes])?;
        self._and()
    }

    /// Load 1 byte to extend self as another number type.
    ///
    /// Just for adapting the WASM instructions, this method makes
    /// no sense for EVM since all of the numbers as U256.
    pub(crate) fn _load8(&mut self, arg: MemArg) -> Result<()> {
        self._load_bytes(arg, 1)
    }

    /// Load 2 bytes to extend self as another number type.
    ///
    /// Just for adapting the WASM instructions, this method makes
    /// no sense for EVM since all of the numbers as U256.
    pub(crate) fn _load16(&mut self, arg: MemArg) -> Result<()> {
        self._load_bytes(arg, 2)
    }

    /// Load 4 bytes to extend self as another number type.
    ///
    /// Just for adapting the WASM instructions, this method makes
    /// no sense for EVM since all of the numbers as U256.
    pub(crate) fn _load32(&mut self, arg: MemArg) -> Result<()> {
        self._load_bytes(arg, 4)
    }

    /// Store n bytes in memory.
    pub fn _store(&mut self, _: MemArg) -> Result<()> {
        todo!()
    }

    /// Wrap self to i8 and store 1 byte
    pub fn _store8(&mut self, _: MemArg) -> Result<()> {
        todo!()
    }

    /// Wrap self to i16 and store 2 bytes
    pub fn _store16(&mut self, _: MemArg) -> Result<()> {
        todo!()
    }

    /// Wrap self to i32 and store 4 bytes
    pub fn _store32(&mut self, _: MemArg) -> Result<()> {
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
