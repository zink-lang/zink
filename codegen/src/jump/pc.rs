//! Program counter handlers.

use crate::{
    jump::{relocate, Jump, JumpTable},
    Error, Result, BUFFER_LIMIT,
};

impl JumpTable {
    /// Shift program counter for all items.
    pub fn shift_pc(&mut self, start: u16, offset: u16) -> Result<()> {
        tracing::trace!("shift pc from 0x{start:x} with offset={offset}");
        self.shift_label_pc(start, offset)?;
        self.shift_label_target(start, offset)?;
        self.shift_func_target(start, offset)?;

        Ok(())
    }

    /// Shift program counter for labels.
    pub fn shift_label_pc(&mut self, start: u16, offset: u16) -> Result<()> {
        self.jump = self
            .jump
            .iter()
            .map(|(k, v)| {
                let mut k = *k;
                if k > start {
                    let next_offset = k + offset;
                    if k > BUFFER_LIMIT as u16 {
                        return Err(Error::InvalidPC(k as usize));
                    }

                    tracing::trace!(
                        "shift {v:x?} pc with offset={offset}: 0x{k:x}(0x{start:x}) -> 0x{:x}",
                        next_offset
                    );
                    k = next_offset;
                }

                Ok((k, v.clone()))
            })
            .collect::<Result<_>>()?;

        Ok(())
    }

    /// Shift the target program counters.
    ///
    /// Calculating target pc from the offset of original pc.
    pub fn shift_targets(&mut self) -> Result<()> {
        let mut total_offset = 0;
        self.jump
            .clone()
            .keys()
            .try_for_each(|original_pc| -> Result<()> {
                let pc = original_pc + total_offset;
                let offset = relocate::offset(pc)?;
                total_offset += offset;
                self.shift_target(pc, offset)
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

    /// Shift program counter for functions.
    pub fn shift_func_target(&mut self, start: u16, offset: u16) -> Result<()> {
        if self.func.is_empty() {
            tracing::trace!("No functions to shift.");
            return Ok(());
        }

        self.func.iter_mut().try_for_each(|(k, v)| {
            if *v > start {
                tracing::trace!(
                    "shift Func({k}) target with offset={offset}: 0x{v:x}(0x{start:x}) -> 0x{:x}",
                    *v + offset
                );
                *v += offset;
                if *v > BUFFER_LIMIT as u16 {
                    return Err(Error::InvalidPC(*v as usize));
                }
            }

            Ok(())
        })?;

        Ok(())
    }

    /// Shift target program counter for labels.
    pub fn shift_label_target(&mut self, ptr: u16, offset: u16) -> Result<()> {
        if self.jump.is_empty() {
            tracing::trace!("No labels to shift.");
            return Ok(());
        }

        self.jump.iter_mut().try_for_each(|(pc, target)| {
            if let Jump::Label(label) = target {
                if *label > ptr {
                    tracing::trace!(
                        "shift Label(pc=0x{pc:x}) target with offset={offset} 0x{label:x}(0x{ptr:x}) -> 0x{:x}",
                        *label + offset
                    );
                    *label += offset;
                }
            }

            Ok(())
        })
    }
}
