//! Control flow visitors

use crate::{
    abi::ToLSBytes,
    control::{ControlStackFrame, ControlStackFrameType},
    CodeGen, Error, Result,
};
use wasmparser::{BlockType, BrTable};

impl CodeGen {
    /// The beginning of an if construct with an implicit block.
    pub fn _if(&mut self, blockty: BlockType) -> Result<()> {
        // push an `If` frame to the control stack
        let frame =
            ControlStackFrame::new(ControlStackFrameType::If, self.masm.pc_offset(), blockty);
        self.control.push(frame);

        // mock the stack output of the counter
        //
        // the program counter operators should be patched afterwards.
        self.masm.asm.increment_sp(1)?;
        self.masm._jumpi()?;

        Ok(())
    }

    /// The begeinning of a block construct. A sequence of
    /// instructions with a label at the end.
    pub fn _block(&mut self, _blockty: BlockType) -> Result<()> {
        todo!()
    }

    /// A block with a label which may be used to
    /// form loops.
    pub fn _loop(&mut self, blockty: BlockType) -> Result<()> {
        let frame =
            ControlStackFrame::new(ControlStackFrameType::Loop, self.masm.pc_offset(), blockty);
        self.control.push(frame);

        Ok(())
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

    /// Handle the end of instructions for different situations.
    ///
    /// TODO: (#28)
    ///
    /// - End of control flow operators.
    /// - End of function.
    /// - End of program.
    pub fn _end(&mut self) -> Result<()> {
        if !self.is_main {
            // TODO: handle the length of results > u8::MAX.
            self.masm.shift_pc(self.env.results().len() as u8, false)?;
            self.masm.push(&[0x04])?;
            self.masm._add()?;
            self.masm._jump()?;
            return Ok(());
        }

        // If inside an if frame, pop the frame and patch
        // the program counter.
        if let Ok(frame) = self.control.pop() {
            self.table
                .label(frame.original_pc_offset, self.masm.pc_offset())?;

            // TODO: Check the stack output and make decisions
            // how to handle the results.

            // Emit JUMPDEST after at the end of the control flow.
            self.masm._jumpdest()?;
        } else {
            let size = self.masm.memory_write(self.env.results())?;
            let offset = self
                .masm
                .mp_offset(|mp| mp.checked_sub(size).ok_or_else(|| Error::InvalidMP(0)))?;

            self.masm.push(&size.to_ls_bytes())?;
            self.masm.push(&offset)?;
            self.masm.asm._return()?;
        }

        Ok(())
    }

    /// Mark as invalid for now.
    ///
    /// TODO: recheck this implementation, if it is okay,
    /// provide more docs.
    pub fn _unreachable(&mut self) -> Result<()> {
        self.masm._invalid()?;
        Ok(())
    }

    /// Perform nothing in EVM bytecode.
    pub fn _nop(&mut self) -> Result<()> {
        Ok(())
    }
}
