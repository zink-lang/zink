//! Program counter handlers.

use crate::{
    jump::{Jump, JumpTable},
    Error, Result, BUFFER_LIMIT,
};

impl JumpTable {
    /// Shift program counter for all items.
    pub fn shift_pc(&mut self, start: u16, offset: u16) -> Result<()> {
        tracing::debug!("shift pc: {}", offset);
        self.shift_label_pc(start, offset)?;
        self.shift_func_pc(start, offset)?;

        Ok(())
    }

    /// Shift program counter for labels.
    pub fn shift_label_pc(&mut self, start: u16, offset: u16) -> Result<()> {
        self.jump = self
            .jump
            .iter()
            .map(|(k, v)| {
                let (mut k, mut v) = (*k, *v);
                if k > start {
                    k = k + offset;
                    if k > BUFFER_LIMIT as u16 {
                        return Err(Error::InvalidPC(k as usize));
                    }
                }

                if let Jump::Label(label) = v {
                    if label > start {
                        v = Jump::Label(label + offset);
                    }
                }

                Ok((k, v))
            })
            .collect::<Result<_>>()?;

        Ok(())
    }

    /// Shift program counter for functions.
    pub fn shift_func_pc(&mut self, start: u16, offset: u16) -> Result<()> {
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
}
