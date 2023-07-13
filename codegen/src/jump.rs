//! Jump table implementation.

use crate::{Buffer, Error, Result, BUFFER_LIMIT};
use std::collections::BTreeMap;

/// Jump types
#[derive(Clone, Copy)]
pub enum Jump {
    /// Jump to the given label, the label here is the original
    /// program counter.
    Label(u16),
    /// Jump to function.
    Func(u32),
}

/// Jump table implementation.
#[derive(Default)]
pub struct JumpTable {
    /// Jump table.
    jump: BTreeMap<u16, Jump>,
    /// Function table.
    func: BTreeMap<u32, u16>,
}

impl JumpTable {
    /// Register a function.
    pub fn call(&mut self, pc: u16, func: u32) -> Result<()> {
        self.jump.insert(pc, Jump::Func(func));
        Ok(())
    }

    /// Register program counter to the function table.
    pub fn call_offset(&mut self, func: u32, offset: u16) -> Result<()> {
        if self.func.insert(func, offset).is_some() {
            return Err(Error::DuplicateFunc(func));
        }

        Ok(())
    }

    /// Register a label.
    pub fn label(&mut self, pc: u16, label: u16) -> Result<()> {
        self.jump.insert(pc, Jump::Label(label));
        Ok(())
    }

    /// Merge two jump tables.
    ///
    /// Merge other jump table into this one, update the program
    /// counter of the target jump table.
    pub fn merge(&mut self, mut table: Self, pc: u16) -> Result<()> {
        tracing::trace!("merge jump table");
        table.update_pc(pc as usize)?;

        for (pc, jump) in table.jump.into_iter() {
            if self.jump.insert(pc, jump).is_some() {
                return Err(Error::DuplicateJump(pc));
            }
        }

        for (func, offset) in table.func.into_iter() {
            if self.func.insert(func, offset).is_some() {
                return Err(Error::DuplicateFunc(func));
            }
        }

        Ok(())
    }

    /// Patch program counter to all registered labels.
    pub fn patch(&mut self, buffer: &mut Buffer) -> Result<()> {
        tracing::trace!("patching jump table");
        while let Some((pc, jump)) = self.jump.pop_first() {
            let target = match jump {
                Jump::Label(label) => label,
                Jump::Func(func) => *self.func.get(&func).ok_or(Error::FuncNotFound(func))?,
            };

            tracing::trace!("jump target: {target}");
            tracing::trace!("buffer: {:x?}", buffer);

            self.update_pc(crate::patch(buffer, pc as usize, target as usize)?)?;
        }

        Ok(())
    }

    /// Update program counter for all items.
    pub fn update_pc(&mut self, pc: usize) -> Result<()> {
        let pc: u16 = pc.try_into().map_err(|_| Error::InvalidPC(pc))?;
        tracing::trace!("update pc to {}", pc);

        self.jump = self
            .jump
            .iter()
            .map(|(k, v)| {
                let k = k + pc;
                if k > BUFFER_LIMIT as u16 {
                    return Err(Error::InvalidPC(k as usize));
                }

                Ok((k, *v))
            })
            .collect::<Result<_>>()?;

        self.func = self
            .func
            .iter()
            .map(|(k, v)| {
                let v = v + pc;
                if v > BUFFER_LIMIT as u16 {
                    return Err(Error::InvalidPC(v as usize));
                }
                Ok((*k, v))
            })
            .collect::<Result<_>>()?;

        Ok(())
    }
}
