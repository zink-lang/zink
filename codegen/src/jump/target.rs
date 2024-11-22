//! target related operations of the jump table

use crate::{
    jump::{Jump, JumpTable},
    Error, Result,
};

impl JumpTable {
    /// Get the target of a jump.
    pub fn target(&self, jump: &Jump) -> Result<u16> {
        match jump {
            Jump::Offset(offset) => Ok(*offset),
            Jump::Label(label) => Ok(*label),
            Jump::Func(func) => Ok(*self.func.get(func).ok_or(Error::FuncNotFound(*func))?),
            Jump::ExtFunc(ext) => Ok(self.code.offset_of(ext).ok_or(Error::ExtFuncNotFound)?),
        }
    }

    /// Shift the target program counters as the pre-calculation.
    ///
    /// Calculating target pc from the offset of original pc and target.
    pub fn shift_targets(&mut self) -> Result<()> {
        let mut total_offset = 0;
        for (original_pc, jump) in self.jump.clone().iter() {
            tracing::debug!("shift targets for {jump} <- (0x{original_pc:x})");
            let pc = original_pc + total_offset;

            // calculate the offset of the target PC, if target > 0xff, it requires
            // 3 bytes for processing the JUMP instruction
            let offset = if self.target(jump)? + total_offset > 0xff {
                3 // [PUSH2, [0x0100..0xffff]]
            } else {
                2 // [PUSH1, [0x00..0xff]]
            };

            self.shift_target(pc, offset)?;
            total_offset += offset;
        }
        Ok(())
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
                }
            }

            Ok(())
        })
    }
}
