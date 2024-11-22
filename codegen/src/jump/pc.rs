//! Program counter handlers.

use crate::{jump::JumpTable, Error, Result, BUFFER_LIMIT};

impl JumpTable {
    /// Shift program counter for all items.
    pub fn shift_pc(&mut self, start: u16, offset: u16) -> Result<()> {
        tracing::trace!("shift pc from 0x{start:x} with offset={offset}");
        self.shift_label_pc(start, offset)?;
        self.shift_label_target(start, offset)?;
        self.shift_func_target(start, offset)
    }

    /// Shift program counter for labels.
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
