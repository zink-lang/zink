//! Control flow visitors

use crate::{
    control::{ControlStackFrame, ControlStackFrameType},
    Error, Function, Result,
};
use wasmparser::{BlockType, BrTable};

impl Function {
    /// The beginning of an if construct with an implicit block.
    pub fn _if(&mut self, blockty: BlockType) -> Result<()> {
        // Emit iszero to check the condition.
        self.masm._iszero()?;

        // push an `If` frame to the control stack
        let frame = ControlStackFrame::new(
            ControlStackFrameType::If(false),
            self.masm.pc(),
            self.masm.sp(),
            blockty,
        );
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
    pub fn _block(&mut self, blockty: BlockType) -> Result<()> {
        let frame = ControlStackFrame::new(
            ControlStackFrameType::Block,
            self.masm.pc(),
            self.masm.sp(),
            blockty,
        );
        self.masm._jumpdest()?;
        self.control.push(frame);

        Ok(())
    }

    /// A block with a label which may be used to
    /// form loops.
    pub fn _loop(&mut self, blockty: BlockType) -> Result<()> {
        let frame = ControlStackFrame::new(
            ControlStackFrameType::Loop,
            self.masm.pc(),
            self.masm.sp(),
            blockty,
        );

        self.masm._jumpdest()?;
        self.control.push(frame);

        Ok(())
    }

    /// Marks an else block of an if.
    pub fn _else(&mut self) -> Result<()> {
        let last_frame = self.control.mark_else()?;
        let frame = ControlStackFrame::new(
            ControlStackFrameType::Else,
            self.masm.pc(),
            self.masm.sp(),
            last_frame.result(),
        );
        self.control.push(frame);

        // Jump to end of if-else block
        self.masm.increment_sp(1)?; // For JUMP
        self.masm._jump()?;
        self.table
            .label(last_frame.original_pc_offset, self.masm.pc());
        self.masm._jumpdest()?;

        Ok(())
    }

    /// The select instruction selects one of its first two operands based
    /// on whether its third oprand is zero or not.
    ///
    /// STACK: [cond, val2, val1] -> \[val1\] if cond is non-zero, \[val2\] otherwise.
    pub fn _select(&mut self) -> Result<()> {
        tracing::trace!("select");
        self.masm._iszero()?;
        self.masm.increment_sp(1)?;
        self.table.label(self.masm.pc(), self.masm.pc() + 2);
        self.masm._jumpi()?;
        self.masm._drop()?;
        self.masm._jumpdest()?;

        Ok(())
    }

    /// Branch to a given label in an enclosing construct.
    ///
    /// Performs an unconditional branch.
    pub fn _br(&mut self, depth: u32) -> Result<()> {
        // Get the target label
        let label = self.control.label_from_depth(depth)?;

        // Check if this is a branch that would exit the function
        let _is_exit_branch = self.control.is_exit_branch(depth);

        // Mark affected frames as having potential early returns
        self.control.mark_frames_with_early_return(depth);

        // Set up jump target in the jump table
        self.table.label(self.masm.pc(), label);

        // Emit unconditional jump instruction
        self.masm._jump()?;

        Ok(())
    }

    /// Performs a conditional branch if i32 is non-zero.
    ///
    /// Conditional branch to a given label in an enclosing construct.
    pub fn _br_if(&mut self, depth: u32) -> Result<()> {
        let label = self.control.label_from_depth(depth)?;

        // Register the jump target in the jump table
        self.table.label(self.masm.pc(), label);

        // for a conditional branch, we need to:
        //
        // increment the stack pointer (for JUMPI's arguments) and
        self.masm.increment_sp(1)?;

        // emit the conditional jump instruction
        self.masm._jumpi()?;

        Ok(())
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
    /// - End of control flow operators.
    /// - End of function.
    /// - End of program.
    pub fn _end(&mut self) -> Result<()> {
        tracing::trace!("ENTERING _end, sp: {}", self.masm.sp());
        if !self.control.is_empty() {
            if let Ok(frame) = self.control.pop() {
                return self.handle_frame_popping(frame);
            }
        }

        let expected_sp = self.returns
            + if self.is_main || self.abi.is_some() {
                0
            } else {
                1
            };
        while self.masm.sp() > expected_sp {
            self.masm._drop()?;
        }
        if self.masm.sp() < expected_sp {
            return Err(Error::StackMismatch {
                expected: expected_sp,
                found: self.masm.sp(),
            });
        }

        let results = self.ty.results();
        if self.is_main || self.abi.is_some() {
            tracing::trace!("end of main function");
            self.masm.main_return(results)?;
        } else {
            tracing::trace!("end of call");
            self.masm.call_return(results)?;
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

    /// Handle the popping of a frame.
    ///
    /// TODO: validate stack IO for all frames (#59)
    pub(crate) fn handle_frame_popping(&mut self, frame: ControlStackFrame) -> Result<()> {
        match frame.ty {
            ControlStackFrameType::If(true) => Ok(()),
            ControlStackFrameType::Block => {
                // For blocks that might have early returns, ensure proper jump destination
                if frame.might_return_early {
                    // Make sure the jump table has this position as a target for the block
                    self.table.label(frame.original_pc_offset, self.masm.pc());
                }
                self.masm._jumpdest()
            }
            ControlStackFrameType::Loop => Ok(()),
            _ => {
                self.table.label(frame.original_pc_offset, self.masm.pc());

                // TODO: Check the stack output and make decisions
                // how to handle the results.

                // Emit JUMPDEST at the end of the control flow.
                self.masm._jumpdest()
            }
        }
    }
}
