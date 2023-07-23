//! Data structures for control flow emission.
use crate::{Error, Result};
use smallvec::SmallVec;
use wasmparser::BlockType;

/// The type of the control stack frame.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ControlStackFrameType {
    /// The if control stack frame.
    ///
    /// true is has else block, otherwise false.
    If(bool),
    /// The else control stack frame.
    Else,
    /// The loop control stack frame.
    Loop,
    /// The block control stack frame.
    Block,
}

/// Holds the necessary metadata to support the smission
/// of control flow instructions.
///
/// NOTE: The output of control flow should be placed on
/// the stack, so we don't need to store the result type.
#[derive(Clone)]
pub struct ControlStackFrame {
    /// The type of the control stack frame.
    ///
    /// If loop, break it while popping.
    ty: ControlStackFrameType,
    /// The program counter offset at the beginning of if.
    pub original_pc_offset: u16,
    /// The return values of the block.
    ///
    /// Could be useful for validation.
    result: BlockType,
}

impl ControlStackFrame {
    /// Create a new control stack frame.
    pub fn new(ty: ControlStackFrameType, original_pc_offset: u16, result: BlockType) -> Self {
        Self {
            ty,
            original_pc_offset,
            result,
        }
    }

    /// Get the offset of the orginal program counter.
    pub fn pc_offset(&self) -> u16 {
        self.original_pc_offset
    }

    /// Get the result type of the control stack frame.
    pub fn result(&self) -> BlockType {
        self.result
    }

    /// Check if the control stack frame is an if block with else.
    pub fn if_with_else(&self) -> bool {
        self.ty == ControlStackFrameType::If(true)
    }
}

/// The control stack.
#[derive(Default)]
pub struct ControlStack {
    /// Stack frames for control flow.
    ///
    /// The 32 is set arbitrarily, we can adjust it as we see fit.
    stack: SmallVec<[ControlStackFrame; 32]>,
}

impl ControlStack {
    /// The total depth of the control stack.
    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    /// Mark the else block of an if.
    pub fn mark_else(&mut self) -> Result<ControlStackFrame> {
        let last = self
            .stack
            .last_mut()
            .ok_or_else(|| Error::ControlStackUnderflow)?;

        if last.ty != ControlStackFrameType::If(false) {
            return Err(Error::InvalidElseBlock(last.original_pc_offset));
        }

        last.ty = ControlStackFrameType::If(true);
        Ok(last.clone())
    }

    /// Push a control stack frame.
    pub fn push(&mut self, frame: ControlStackFrame) {
        self.stack.push(frame);
    }

    /// Pop a control stack frame.
    pub fn pop(&mut self) -> Result<ControlStackFrame> {
        self.stack.pop().ok_or_else(|| Error::ControlStackUnderflow)
    }

    /// Get the label of the control stack frame at given depth.
    pub fn label_from_depth(&self, mut depth: u32) -> Result<u16> {
        for frame in self.stack.iter().rev() {
            if frame.ty == ControlStackFrameType::Else {
                continue;
            }

            if depth == 0 {
                return Ok(frame.pc_offset());
            }

            depth -= 1;
        }

        Err(Error::InvalidDepth(depth as usize))
    }

    /// Get the return type of the control stack frame at given depth.
    pub fn ret_ty(&self, depth: usize) -> Result<BlockType> {
        if depth == 0 {
            return Err(Error::InvalidDepth(depth));
        }

        self.stack
            .get(self.depth() - depth)
            .map(|f| f.result)
            .ok_or_else(|| Error::InvalidDepth(depth))
    }

    /// Get the type of the control stack frame at given depth.
    pub fn ty(&self, depth: usize) -> Result<ControlStackFrameType> {
        if depth == 0 {
            return Err(Error::InvalidDepth(depth));
        }

        self.stack
            .get(self.depth() - depth)
            .map(|f| f.ty)
            .ok_or_else(|| Error::InvalidDepth(depth))
    }
}
