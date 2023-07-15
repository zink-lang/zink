//! Stack Instructions

use crate::{MacroAssembler, Result};

impl MacroAssembler {
    /// The drop instruction simply throw away a single operand.
    pub fn _drop(&mut self) -> Result<()> {
        self._pop()
    }
}
