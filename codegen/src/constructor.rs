//! Contract constructor.

use crate::{MacroAssembler, Result};

/// Contract constructor.
///
/// # Bytecode
/// - `CREATE` instruction
/// - `INIT_CODE`
///   - `INIT_LOGIC`
///   - `RETURN RUNTIME_BYTECODE`
/// - `RUNTIME_BYTECODE`
#[derive(Default)]
pub struct Constructor {
    /// Code buffer.
    pub masm: MacroAssembler,
}

impl Constructor {
    /// Concat the constructor code.
    pub fn finish() -> Result<Vec<u8>> {
        Ok(vec![])
    }
}
