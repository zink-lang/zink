//! Target related operations of the jump table.
//!
//! This module provides functions to retrieve and shift the target program counters
//! associated with various jump types.

use crate::{
    jump::{Jump, JumpTable},
    Error, Result,
};

impl JumpTable {
    /// Retrieves the target of a given jump.
    ///
    /// This function returns the target program counter based on the type of jump
    /// (offset, label, function, or external function).
    pub fn target(&self, jump: &Jump) -> Result<u16> {
        match jump {
            Jump::Label(label) => Ok(*label),
            Jump::Func(func) => Ok(*self.func.get(func).ok_or(Error::FuncNotFound(*func))?),
            Jump::ExtFunc(ext) => Ok(self.code.offset_of(ext).ok_or(Error::ExtFuncNotFound)?),
        }
    }

    /// Shifts the target program counters as a pre-calculation step.
    ///
    /// This function calculates the target program counter from the original program
    /// counter and the target, adjusting for any offsets.
    pub fn shift_targets(&mut self) -> Result<()> {
        let mut total_offset = 0;
        for (original_pc, jump) in self.jump.clone().iter() {
            tracing::debug!("shift targets for {jump} <- (0x{original_pc:x})");
            let pc = original_pc + total_offset;

            // Determine the size of the target PC based on its value.
            let target = self.target(jump)? + total_offset;

            /* if jump.is_offset() {
                target += original_pc;
            } */

            let offset = if target > 0xff {
                3 // Requires 3 bytes for processing the JUMP target offset
            } else {
                2 // Requires 2 bytes
            };

            self.shift_target(pc, offset)?;
            total_offset += offset;
        }
        Ok(())
    }

    /// Shifts the program counter of targets with the given pointer and offset.
    ///
    /// This function handles the shifting of the code section, label targets, and
    /// function targets.
    pub fn shift_target(&mut self, ptr: u16, offset: u16) -> Result<()> {
        self.code.shift(offset);
        self.shift_label_target(ptr, offset)?;
        self.shift_func_target(ptr, offset)
    }

    /// Shifts the program counter for functions.
    pub fn shift_func_target(&mut self, ptr: u16, offset: u16) -> Result<()> {
        self.func.iter_mut().try_for_each(|(index, target)| {
            let next_target = *target + offset;

            if *target > ptr {
                tracing::trace!(
                    "shift Func({index}) target with offset={offset}: 0x{target:x}(0x{ptr:x}) -> 0x{:x}",
                    next_target
                );

                *target = next_target;
            }

            Ok(())
        })
    }

    /// Shifts the program counter for labels.
    pub fn shift_label_target(&mut self, ptr: u16, offset: u16) -> Result<()> {
        for (pc, jump) in self.jump.iter_mut() {
            let target: &mut u16 = match jump {
                Jump::Label(target) => target,
                // Jump::Offset { target, .. } => target,
                _ => continue,
            };

            let next_target = *target + offset;
            if *target > ptr {
                tracing::trace!(
                            "shift Label(0x{pc:x}) target with offset={offset}: 0x{target:x}(0x{ptr:x}) -> 0x{:x}",
                            next_target,
                        );

                *target = next_target;
            }
        }
        Ok(())
    }
}
