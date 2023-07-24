//! Program counter handlers.

use crate::{
    jump::{relocate, Jump, JumpTable},
    Error, Result, BUFFER_LIMIT,
};

impl JumpTable {
    /// Shift program counter for all items.
    pub fn shift_pc(&mut self, start: u16, offset: u16) -> Result<()> {
        self.shift_label_pc(start, offset)?;
        self.shift_func_target(start, offset)?;

        Ok(())
    }

    /// Update program counter for all items.
    pub fn update_pc(&mut self) -> Result<()> {
        self.jump
            .clone()
            .iter()
            .try_for_each(|(pc, _jump)| -> Result<()> {
                self.shift_pc(*pc, relocate::offset(*pc)?)?;
                Ok(())
            })
    }

    /// Shift the target program counters.
    pub fn shift_targets(&mut self) -> Result<()> {
        self.jump.clone().keys().try_for_each(|pc| -> Result<()> {
            self.shift_target(*pc, relocate::offset(*pc)?)?;
            Ok(())
        })
    }

    /// Shift the program counter of targets with given ptr and offset.
    pub fn shift_target(&mut self, ptr: u16, offset: u16) -> Result<()> {
        self.shift_label_target(ptr, offset)?;
        self.shift_func_target(ptr, offset)
    }

    /// Shift program counter for functions.
    pub fn shift_func_target(&mut self, start: u16, offset: u16) -> Result<()> {
        self.func.values_mut().try_for_each(|v| {
            if *v > start {
                *v += offset;
                if *v > BUFFER_LIMIT as u16 {
                    return Err(Error::InvalidPC(*v as usize));
                }
            }

            Ok(())
        })?;

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
                    k += offset;
                    if k > BUFFER_LIMIT as u16 {
                        return Err(Error::InvalidPC(k as usize));
                    }
                }

                Ok((k, *v))
            })
            .collect::<Result<_>>()?;

        Ok(())
    }

    /// Shift target program counter for labels.
    pub fn shift_label_target(&mut self, ptr: u16, offset: u16) -> Result<()> {
        self.jump.values_mut().try_for_each(|target| {
            if let Jump::Label(label) = target {
                if *label > ptr {
                    *label += offset;
                }
            }

            Ok(())
        })
    }
}
