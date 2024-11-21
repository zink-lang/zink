//! Program counter handlers.

use crate::{
    jump::{Jump, JumpTable},
    Error, Result, BUFFER_LIMIT,
};

impl JumpTable {
    /// Shift program counter for all items.
    pub fn shift_pc(&mut self, start: u16, offset: u16) -> Result<()> {
        tracing::trace!("shift pc from 0x{start:x} with offset={offset}");
        self.shift_label_pc(start, offset)?;
        self.shift_label_target(start, offset)?;
        self.shift_func_target(start, offset)
    }

    /// Shift the target program counters.
    ///
    /// Calculating target pc from the offset of original pc.
    pub fn shift_targets(&mut self) -> Result<()> {
        let mut total_offset = 0;
        self.jump
            .clone()
            .iter()
            .try_for_each(|(original_pc, target)| -> Result<()> {
                tracing::debug!("shift targets for {target} <- (0x{original_pc:x})");
                let pc = original_pc + total_offset;
                // For offset jumps, we only need the total_offset as they're relative
                let offset = self.target_offset(target, total_offset)?;

                self.shift_target(pc, offset)?;
                total_offset += offset;
                Ok(())
            })
    }

    /// Shift the program counter of targets with given ptr and offset.
    ///
    /// 1. shift code section.
    /// 2. shift label targets.
    /// 3. shift function targets.
    pub fn shift_target(&mut self, ptr: u16, offset: u16) -> Result<()> {
        self.code.shift(offset);
        self.shift_label_target(ptr, offset)?;
        self.shift_func_target(ptr, offset)
    }

    /// Shift program counter for labels.
    pub fn shift_label_pc(&mut self, start: u16, offset: u16) -> Result<()> {
        self.jump = self
            .jump
            .iter()
            .map(|(k, v)| {
                let mut k = *k;
                let next_label = k + offset;
                if k > start {
                    tracing::trace!(
                        "shift {v} pc with offset={offset}: 0x{k:x}(0x{start:x}) -> 0x{:x}",
                        next_label
                    );
                    k = next_label;

                    if k > BUFFER_LIMIT as u16 {
                        return Err(Error::InvalidPC(k as usize));
                    }
                }

                Ok((k, v.clone()))
            })
            .collect::<Result<_>>()?;

        Ok(())
    }

    /// Shift program counter for functions.
    pub fn shift_func_target(&mut self, ptr: u16, offset: u16) -> Result<()> {
        if self.func.is_empty() {
            tracing::trace!("No functions to shift.");
            return Ok(());
        }

        self.func.iter_mut().try_for_each(|(index, target)| {
            let next_target = *target + offset;

            if *target > ptr {
                tracing::trace!(
                    "shift Func({index}) target with offset={offset}: 0x{target:x}(0x{ptr:x}) -> 0x{:x}",
                    next_target
                );

                *target = next_target;
            } else {
                tracing::trace!(
                    "shift Func({index}) target with offset=0: 0x{target:x}(0x{ptr:x}) -> 0x{target:x}"
                );
            }

            Ok(())
        })
    }

    /// Shift target program counter for labels.
    pub fn shift_label_target(&mut self, ptr: u16, offset: u16) -> Result<()> {
        if self.jump.is_empty() {
            tracing::trace!("No labels to shift.");
            return Ok(());
        }

        self.jump.iter_mut().try_for_each(|(pc, target)| {
            if let Jump::Label(label) = target {
                let next_label = *label + offset;

                if *label > ptr {
                    tracing::trace!(
                        "shift Label(0x{pc:x}) target with offset={offset}: 0x{label:x}(0x{ptr:x}) -> 0x{:x}",
                        next_label,
                    );

                    *label = next_label;
                } else {
                    tracing::trace!(
                        "shift Label(0x{pc:x}) target with offset=0: 0x{label:x}(0x{ptr:x}) -> 0x{label:x}"
                    );
                }
            }

            Ok(())
        })
    }
}
