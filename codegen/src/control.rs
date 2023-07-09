//! Data structures for control flow emission.
use crate::{Error, Result};
use smallvec::{smallvec, SmallVec};
use wasmparser::ValType;

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
pub struct ControlStackFrame {
    /// The type of the control stack frame.
    ty: ControlStackFrameType,
    /// The program counter offset at the beginning of if.
    original_pc_offset: u16,
    /// The return values of the block.
    result: SmallVec<[ValType; 1]>,
}

impl ControlStackFrame {
    /// Create a new control stack frame.
    pub fn new(
        ty: ControlStackFrameType,
        original_pc_offset: u16,
        result: SmallVec<[ValType; 1]>,
    ) -> Self {
        Self {
            ty,
            original_pc_offset,
            result,
        }
    }

    /// The maximum code size of a EVM contract is 0x6000 bytes.
    /// so `u16` for the label should be enough.
    pub fn label(&self) -> SmallVec<[u8; 3]> {
        let mut label = smallvec![];
        label.push(self.ty as u8);
        label.extend_from_slice(&self.original_pc_offset.to_le_bytes());

        label
    }

    /// Get the result type of the control stack frame.
    pub fn result(&self) -> SmallVec<[ValType; 1]> {
        self.result.clone()
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
    /// Push a if control stack frame.
    pub fn push_if(&mut self, original_pc_offset: u16, result: SmallVec<[ValType; 1]>) {
        self.stack.push(ControlStackFrame::new(
            ControlStackFrameType::If,
            original_pc_offset,
            result,
        ));
    }

    /// Push an else control stack frame.
    pub fn push_else(&mut self, original_pc_offset: u16, result: SmallVec<[ValType; 1]>) {
        self.stack.push(ControlStackFrame::new(
            ControlStackFrameType::Else,
            original_pc_offset,
            result,
        ));
    }

    /// Push a loop control stack frame.
    pub fn push_loop(&mut self, original_pc_offset: u16, result: SmallVec<[ValType; 1]>) {
        self.stack.push(ControlStackFrame::new(
            ControlStackFrameType::Loop,
            original_pc_offset,
            result,
        ));
    }

    /// Push a block control stack frame.
    pub fn push_block(&mut self, original_pc_offset: u16, result: SmallVec<[ValType; 1]>) {
        self.stack.push(ControlStackFrame::new(
            ControlStackFrameType::Block,
            original_pc_offset,
            result,
        ));
    }

    /// Pop a control stack frame.
    pub fn pop(&mut self) -> Result<ControlStackFrame> {
        self.stack.pop().ok_or_else(|| Error::ControlStackUnderflow)
    }
}
