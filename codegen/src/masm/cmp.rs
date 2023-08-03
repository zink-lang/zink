// Comparison Instructions

use crate::{MacroAssembler, Result};
use opcodes::ShangHai as OpCode;

impl MacroAssembler {
    /// Greater than or equal comparison.
    ///
    /// a b ge -> a b-1 gt(lt)
    ///
    /// Using lt due to order of stack.
    pub fn _ge(&mut self) -> Result<()> {
        self.push(&[1])?;
        // NOTE: this is the overrided sub but not `self.asm.sub`
        self._sub()?;
        self.asm._lt()
    }

    /// Greater than or equal comparison.
    ///
    /// a b sge -> a b-1 sgt(slt)
    ///
    /// Using lt due to order of stack.
    pub fn _sge(&mut self) -> Result<()> {
        self.push(&[1])?;
        // NOTE: this is the overrided sub but not `self.asm.sub`
        self._sub()?;
        self.asm._slt()
    }

    /// Greater than or equal comparison.
    ///
    /// a b sge -> a b-1 sgt(slt)
    ///
    /// Using lt due to order of stack.
    pub fn _sle(&mut self) -> Result<()> {
        self.push(&[1])?;
        // NOTE: this is the overrided sub but not `self.asm.sub`
        self._sub()?;
        self.asm._slt()
    }

    /// Greater than or equal comparison.
    ///
    /// a b le -> a b-1 lt(gt)
    ///
    /// Using gt due to order of stack.
    pub fn _le(&mut self) -> Result<()> {
        self.push(&[1])?;
        // NOTE: this is the overrided sub but not `self.asm.sub`
        self._sub()?;
        self.asm._lt()
    }

    /// Greater than and equal comparison.
    ///
    /// Using slt due to order of stack.
    pub fn _sgt(&mut self) -> Result<()> {
        self.asm._slt()
    }

    /// Greater than comparison.
    ///
    /// Using lt due to order of stack.
    pub fn _gt(&mut self) -> Result<()> {
        self.asm._lt()
    }

    /// less than comparison.
    ///
    /// Using gt due to order of stack.
    pub fn _lt(&mut self) -> Result<()> {
        self.asm._gt()
    }

    /// less than or equal comparison.
    ///
    /// Using gt due to order of stack.
    pub fn _slt(&mut self) -> Result<()> {
        self.asm._sgt()
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
