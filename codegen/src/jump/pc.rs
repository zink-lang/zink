//! Program counter handlers.
//!
//! This module provides functionality to shift the program counter for various jump types
//! and manage the relationships between labels and their corresponding program counters.

use crate::{jump::JumpTable, Error, Result, BUFFER_LIMIT};

impl JumpTable {
    /// Shifts the program counter for all jump items.
    ///
    /// This function updates the program counters based on a starting point and an offset.
    pub fn shift_pc(&mut self, start: u16, offset: u16) -> Result<()> {
        tracing::trace!("shift pc from 0x{start:x} with offset={offset}");
        self.shift_label_pc(start, offset)?;
        self.shift_label_target(start, offset)?;
        self.shift_func_target(start, offset)
    }

    /// Shifts the program counter for labels.
    ///
    /// This function updates the program counters of labels based on the specified start
    /// point and offset.
    pub fn shift_label_pc(&mut self, start: u16, offset: u16) -> Result<()> {
        let mut new_jump = Vec::new();
        for (label, jump) in self.jump.iter() {
            let mut label = *label;
            let next_label = label + offset;
            if label > start {
                tracing::trace!(
                    "shift {jump} pc with offset={offset}: 0x{label:x}(0x{start:x}) -> 0x{:x}",
                    next_label
                );
                label = next_label;

                if label > BUFFER_LIMIT as u16 {
                    return Err(Error::InvalidPC(label as usize));
                }
            }

            new_jump.push((label, jump.clone()));
        }

        self.jump = new_jump.into_iter().collect();
        Ok(())
    }
}
