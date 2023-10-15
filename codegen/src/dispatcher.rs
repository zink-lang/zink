//! Code generator for EVM dispatcher.

use crate::{Buffer, Exports, Imports, JumpTable, Result};

/// Code generator for EVM dispatcher.
#[derive(Default)]
pub struct Dispatcher {
    /// Code buffer
    pub buffer: Buffer,
    /// Module exports
    pub exports: Exports,
    /// Module imports
    pub imports: Imports,
    /// Jump table
    pub table: JumpTable,
}

impl Dispatcher {
    /// Set exports for the dispatcher.
    pub fn exports(&mut self, exports: Exports) -> &mut Self {
        self.exports = exports;
        self
    }

    /// Set imports for the dispatcher.
    pub fn imports(&mut self, imports: Imports) -> &mut Self {
        self.imports = imports;
        self
    }

    /// Emit compiled code to the given buffer.
    pub fn finish(&mut self, _table: &mut JumpTable) -> Result<Vec<u8>> {
        Ok(vec![])
    }
}
