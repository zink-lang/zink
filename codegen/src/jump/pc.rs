//! Program counter handlers.

use crate::{
    jump::{Jump, JumpTable},
    Error, Result, BUFFER_LIMIT,
};

impl JumpTable {
    /// Update program counter for labels.
    pub fn update_label_pc(&mut self, pc: usize) -> Result<()> {
        let pc: u16 = pc.try_into().map_err(|_| Error::InvalidPC(pc))?;
        self.jump = self
            .jump
            .iter()
            .map(|(k, v)| {
                let k = k + pc;
                if k > BUFFER_LIMIT as u16 {
                    return Err(Error::InvalidPC(k as usize));
                }

                // NOTE: we don't need to update the function PC before
                // the current PC (jump back)

                let v = match v {
                    Jump::Label(label) => Jump::Label(label + pc),
                    Jump::Func(func) => Jump::Func(*func),
                };

                Ok((k, v))
            })
            .collect::<Result<_>>()?;

        Ok(())
    }

    /// Update program counter of functions.
    pub fn update_func_pc(&mut self, pc: usize) -> Result<()> {
        let pc: u16 = pc.try_into().map_err(|_| Error::InvalidPC(pc))?;
        self.func
            .values_mut()
            .map(|v| {
                let v = *v + pc;
                if v > BUFFER_LIMIT as u16 {
                    return Err(Error::InvalidPC(v as usize));
                }
                Ok(())
            })
            .collect::<Result<()>>()?;

        Ok(())
    }

    /// Update program counter for all items.
    pub fn update_pc(&mut self, pc: usize) -> Result<()> {
        tracing::debug!("update pc: {}", pc);
        self.update_label_pc(pc)?;
        self.update_func_pc(pc)?;

        Ok(())
    }
}
