//! Embedded Function implementations

use crate::{MacroAssembler, Result};

impl MacroAssembler {
    /// Function `sstore` from EVM which is not available in WASM.
    pub fn _sstore(&mut self) -> Result<()> {
        self.asm._sstore()
    }

    /// Function `sload` from EVM which is not available in WASM.
    pub fn _sload(&mut self) -> Result<()> {
        self.asm._sload()
    }
}
