// Comparison Instructions

use crate::{MacroAssembler, Result};
use opcodes::ShangHai as OpCode;

impl MacroAssembler {
    /// Greater than or equal comparison.
    ///
    /// TODO: refactor this.
    pub fn _ge(&mut self) -> Result<()> {
        self._sgt()
    }

    /// Greater than comparison.
    ///
    /// Using lt due to order of stack.
    pub fn _gt(&mut self) -> Result<()> {
        self.asm._lt()
    }

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
