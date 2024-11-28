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
        let mut target_sizes = Vec::new();

        // First pass: calculate all target sizes
        for (original_pc, jump) in self.jump.clone().iter() {
            let pc = original_pc + total_offset;
            let target = self.target(jump)? + total_offset;

            // Calculate instruction size based on target value
            let instr_size = if target > 0xff {
                3 // PUSH2 + 2 bytes
            } else {
                2 // PUSH1 + 1 byte
            };

            target_sizes.push((pc, instr_size));
            total_offset += instr_size;
        }

        // Second pass: apply shifts with correct accumulated offsets
        total_offset = 0;
        for (pc, size) in target_sizes {
            tracing::debug!("shift target at pc=0x{pc:x} with size={size}");
            self.shift_target(pc, size)?;
            total_offset += size;
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
            let Jump::Label(target) = jump else {
                continue;
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
