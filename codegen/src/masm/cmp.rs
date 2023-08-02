// Comparison Instructions

use crate::{MacroAssembler, Result};
use opcodes::ShangHai as OpCode;

impl MacroAssembler {
    // /// OVERRIDE: Greater than comparison.
    // ///
    // /// TODO:
    // pub fn _sgt(&mut self) -> Result<()> {
    //     self._swap1()?;
    //     self.asm._sgt()
    // }

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
