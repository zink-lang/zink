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
    pub ty: ControlStackFrameType,
    /// The program counter offset at the beginning of if.
    pub original_pc_offset: u16,
    /// The return values of the block.
    ///
    /// Could be useful for validation.
    result: BlockType,

    /// Original stack pointer.
    pub original_sp: u16,
    
    /// Flag to mark frames that might contain early returns
    pub might_return_early: bool,
}

impl ControlStackFrame {
    /// Create a new control stack frame.
    pub fn new(
        ty: ControlStackFrameType,
        original_pc_offset: u16,
        original_sp: u16,
        result: BlockType,
    ) -> Self {
        Self {
            ty,
            original_pc_offset,
            original_sp,
            result,
            might_return_early: false,
        }
    }

    /// Get the offset of the original program counter.
    pub fn pc_offset(&self) -> u16 {
        self.original_pc_offset
    }

    /// Get the result type of the control stack frame.
    pub fn result(&self) -> BlockType {
        self.result
    }
    
    /// Set the flag indicating this frame might contain early returns
    pub fn set_might_return_early(&mut self, value: bool) {
        self.might_return_early = value;
    }
    
    /// Check if this frame is at a function boundary
    pub fn is_function_boundary(&self) -> bool {
        // For now, we consider Block frames as potential function boundaries
        matches!(self.ty, ControlStackFrameType::Block)
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
    
    /// Get a reference to the frame at the given depth
    pub fn frame_from_depth(&self, mut depth: u32) -> Result<&ControlStackFrame> {
        for (i, frame) in self.stack.iter().rev().enumerate() {
            if frame.ty == ControlStackFrameType::Else {
                continue;
            }

            if depth == 0 {
                return Ok(&self.stack[self.stack.len() - 1 - i]);
            }

            depth -= 1;
        }

        Err(Error::InvalidDepth(depth as usize))
    }
    
    /// Check if a branch at the given depth would exit the function
    pub fn is_exit_branch(&self, depth: u32) -> bool {
        // If depth exceeds our stack, it's definitely an exit
        if depth as usize >= self.depth() {
            return true;
        }
        
        // Get the frame that would be the target
        if let Ok(frame) = self.frame_from_depth(depth) {
            // If it's a Block at the outermost level (index 0), it might be a function boundary
            if matches!(frame.ty, ControlStackFrameType::Block) {
                for (i, f) in self.stack.iter().enumerate() {
                    if f.original_pc_offset == frame.original_pc_offset {
                        return i == 0; // It's an exit if it's the outermost block
                    }
                }
            }
        }
        
        false
    }
    
    /// Mark frames as potentially having early returns
    pub fn mark_frames_with_early_return(&mut self, depth: u32) {
        let target_idx = if depth as usize >= self.depth() {
            // If targeting beyond the stack, mark all frames
            0 // Start from the first frame
        } else {
            // Find the target frame index
            let mut target_idx = self.depth();
            let mut current_depth = 0;
            
            for (i, frame) in self.stack.iter().rev().enumerate() {
                if frame.ty == ControlStackFrameType::Else {
                    continue;
                }
                
                if current_depth == depth {
                    target_idx = self.depth() - 1 - i;
                    break;
                }
                
                current_depth += 1;
            }
            
            target_idx
        };
        
        // Mark frames from target to the end
        for i in target_idx..self.depth() {
            self.stack[i].set_might_return_early(true);
        }
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
    
    /// Get the length of the control stack
    pub fn len(&self) -> usize {
        self.stack.len()
    }
    
    /// Check if the control stack is empty
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
