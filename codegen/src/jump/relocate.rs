//! Program Relocations
//!
//! This module provides functionality for relocating program counters associated
//! with various jump types in the jump table. It handles the adjustment of
//! program counters based on their target locations, ensuring that jumps
//! point to the correct addresses after any modifications to the code section.

use crate::{
    jump::{relocate, JumpTable},
    wasm::ToLSBytes,
    Buffer, Error, Result, BUFFER_LIMIT,
};
use opcodes::ShangHai as OpCode;

impl JumpTable {
    /// Relocate program counter to all registered labels.
    ///
    /// This function is responsible for adjusting the program counters of all
    /// jumps in the jump table. It first pre-calculates and shifts the target
    /// program counters, then iterates through each jump to relocate its
    /// target address. The function ensures that the jumps are correctly
    /// updated in the buffer, which represents the compiled code.
    ///
    /// *WARNING*: This function should only be called once in the compiler.
    /// Consider moving it to the compiler's main logic.
    pub fn relocate(&mut self, buffer: &mut Buffer) -> Result<()> {
        // Pre-calculate and shift targets to ensure all jumps point to the correct addresses.
        self.shift_targets()?;
        tracing::trace!("code section offset: 0x{:x}", self.code.offset());

        // Relocate each function in the jump table.
        while let Some((pc, jump)) = self.jump.pop_first() {
            tracing::debug!(
                "Relocating jump {:?} at pc=0x{:x}, current_offset=0x{:x}",
                jump,
                pc,
                self.code.offset()
            );
            let mut target = self.target(&jump)?;

            // If the jump is an offset, adjust the target accordingly.
            if jump.is_offset() {
                self.relocate_offset(pc, &mut target)?;
            }

            tracing::debug!(
                "relocate: pc=0x{:x}, jump={:?}, target=0x{:x}",
                pc,
                jump,
                target
            );

            // Update the buffer with the new target program counter.
            let offset = relocate::pc(buffer, pc, target)?;
            self.shift_label_pc(pc, offset as u16)?;
        }

        // Extend the buffer with the finished code section.
        buffer.extend_from_slice(&self.code.finish());
        Ok(())
    }

    /// Relocate the target of an offset jump.
    ///
    /// This function adjusts the target program counter for jumps that are
    /// represented as offsets. It modifies the target based on the original
    /// program counter and checks for specific conditions that may require
    /// further adjustments.
    fn relocate_offset(&self, pc: u16, target: &mut u16) -> Result<()> {
        // NOTE: If the target is offset, the return data is the offset instead of the PC.
        *target += pc;

        // Check if the original program counter of the offset is greater than 0xff.
        if pc > 0xff {
            *target += 1;
        }

        // Check if the offset of the embedded call is greater than 0xff.
        if let Some((_, next_target)) = self.jump.first_key_value() {
            if next_target.is_call() && self.target(next_target)? > 0xff {
                *target += 1
            }
        }

        Ok(())
    }
}

/// Relocate program counter to buffer.
///
/// This function takes the original program counter and the target program
/// counter, and updates the provided buffer with the necessary opcode
/// instructions. It ensures that the buffer does not exceed the defined
/// size limit and handles the conversion of the target program counter
/// to the appropriate byte representation.
fn pc(buffer: &mut Buffer, original_pc: u16, target_pc: u16) -> Result<usize> {
    let original_pc = original_pc as usize;
    let mut new_buffer: Buffer = buffer[..original_pc].into();
    let rest_buffer: Buffer = buffer[original_pc..].into();

    // Convert the target program counter to its byte representation.
    let pc_offset = target_pc.to_ls_bytes();
    if pc_offset.len() == 1 {
        new_buffer.push(OpCode::PUSH1.into());
    } else {
        new_buffer.push(OpCode::PUSH2.into());
    }

    tracing::trace!(
        "push bytes: 0x{} at 0x{}",
        hex::encode(&pc_offset),
        hex::encode(original_pc.to_ls_bytes())
    );
    new_buffer.extend_from_slice(&pc_offset);
    new_buffer.extend_from_slice(&rest_buffer);

    // Check if the new buffer size exceeds the defined limit.
    if new_buffer.len() > BUFFER_LIMIT {
        return Err(Error::BufferOverflow(new_buffer.len()));
    }

    // Update the original buffer with the new contents.
    *buffer = new_buffer;
    Ok(1 + pc_offset.len())
}
