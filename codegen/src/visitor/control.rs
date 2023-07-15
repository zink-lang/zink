//! Control flow visitors

use crate::{CodeGen, Result};
use wasmparser::{BlockType, BrTable};

impl CodeGen {
    /// The begeinning of a block construct. A sequence of
    /// instructions with a label at the end.
    pub fn _block(&mut self, _blockty: BlockType) -> Result<()> {
        todo!()
    }

    /// A block with a label which may be used to
    /// form loops.
    pub fn _loop(&mut self, _blockty: BlockType) -> Result<()> {
        todo!()
    }

    /// Marks an else block of an if.
    pub fn _else(&mut self) -> Result<()> {
        todo!()
    }

    /// The select instruction selects one of its first two operands based
    /// on whether its third oprand is zero or not.
    pub fn _select(&mut self) -> Result<()> {
        todo!()
    }

    /// Branch to a given label in an enclosing construct.
    ///
    /// Performs an unconditional branch.
    pub fn _br(&mut self, _depth: u32) -> Result<()> {
        todo!()
    }

    /// Performs a conditional branch if i32c is non-zero.
    ///
    /// Conditional branch to a given label in an enclosing construct.
    pub fn _br_if(&mut self, _depth: u32) -> Result<()> {
        todo!()
    }

    /// A jump table which jumps to a label in an enclosing construct.
    ///
    /// Performs an indirect branch through an operand indexing into the
    /// label vector that is an immediate to the instruction, or to the
    /// default target if the operand is out of bounds.
    pub fn _br_table(&mut self, _table: BrTable<'_>) -> Result<()> {
        todo!()
    }
}
