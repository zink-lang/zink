//! Stack Instructions

use crate::{Assembler, Result};

impl Assembler {
    /// The drop instruction simply throw away a single operand.
    pub fn _drop(&mut self) -> Result<()> {
        self._pop()
    }
}
