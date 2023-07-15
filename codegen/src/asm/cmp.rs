// Comparison Instructions

use crate::{Assembler, Result};
use opcodes::ShangHai as OpCode;

impl Assembler {
    /// Sign-agnostic compare unequal.
    pub fn _ne(&mut self) -> Result<()> {
        self.emit_op(OpCode::EQ)?;
        self.emit_op(OpCode::ISZERO)?;
        Ok(())
    }

    /// Simple not operator
    pub fn _eqz(&mut self) -> Result<()> {
        self.emit_op(OpCode::ISZERO)?;
        Ok(())
    }
}
