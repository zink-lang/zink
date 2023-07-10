//! Data structures for control flow emission.
use crate::{Error, Result};
use smallvec::{smallvec, SmallVec};
use wasmparser::BlockType;

/// The type of the control stack frame.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ControlStackFrameType {
    /// The if control stack frame.
    If,
    /// The else control stack frame.
    Else,
    /// The loop control stack frame.
    Loop,
    /// The block control stack frame.
    Block,
}

/// Holds the necessary metadata to support the smission
/// of control flow instructions.
#[derive(Clone)]
pub struct ControlStackFrame {
    /// The type of the control stack frame.
    ty: ControlStackFrameType,
    /// The program counter offset at the beginning of if.
    original_pc_offset: u16,
    /// The return values of the block.
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

    /// The maximum code size of a EVM contract is 0x6000 bytes.
    /// so `u16` for the label should be enough.
    ///
    /// This label will be placed right before the control flow
    /// instruction `JUMP` momentarilly, and be replaced by the
    /// actual jump destination when mactching instruction `End`
    /// afterwards.
    pub fn label(&self) -> SmallVec<[u8; 3]> {
        let mut label = smallvec![];
        label.push(self.ty as u8);
        label.extend_from_slice(&self.original_pc_offset.to_le_bytes());

        label
    }

    /// Get the offset of the orginal program counter.
    pub fn pc_offset(&self) -> u16 {
        self.original_pc_offset
    }

    /// Get the result type of the control stack frame.
    pub fn result(&self) -> BlockType {
        self.result
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
    /// Get the current frame.
    pub fn current(&self) -> Result<ControlStackFrame> {
        Ok(self
            .stack
            .last()
            .cloned()
            .ok_or_else(|| Error::ControlStackUnderflow)?)
    }

    /// Push a block control stack frame.
    pub fn push(&mut self, frame: ControlStackFrame) {
        self.stack.push(frame);
    }

    /// Pop a control stack frame.
    ///
    /// TODO: update the offsets of all frames.
    pub fn pop(&mut self) -> Result<ControlStackFrame> {
        self.stack.pop().ok_or_else(|| Error::ControlStackUnderflow)
    }
}
