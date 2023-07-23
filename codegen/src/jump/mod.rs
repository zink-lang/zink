//! Jump table implementation.

use crate::{Buffer, Error, Result, BUFFER_LIMIT};
use std::collections::BTreeMap;

mod relocate;

/// Jump types
#[derive(Clone, Copy)]
pub enum Jump {
    /// Jump to the given label, the label here is the original
    /// program counter.
    Label(u16),
    /// Jump to function.
    Func(u32),
}

impl Jump {
    /// If the target is a label.
    pub fn is_label(&self) -> bool {
        matches!(self, Jump::Label(_))
    }
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
    pub fn label(&mut self, pc: u16, label: u16) {
        self.jump.insert(pc, Jump::Label(label));
    }

    /// Merge two jump tables.
    ///
    /// Merge other jump table into this one, update the program
    /// counter of the target jump table.
    pub fn merge(&mut self, mut table: Self, pc: u16) -> Result<()> {
        if pc != 0 {
            table.update_pc(pc as usize)?;
        }

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

    /// Relocate program counter to all registered labels.
    pub fn relocate(&mut self, buffer: &mut Buffer) -> Result<()> {
        let mut funcs = BTreeMap::default();
        while let Some((pc, jump)) = self.jump.pop_first() {
            let target = self.target(&jump)?;
            if jump.is_label() {
                let offset = relocate::pc(buffer, pc as usize, target as usize, false)?;

                if pc > target {
                    // TODO:
                    //
                    // 1. check this logic with more tests.
                    // 2. checked add offset.
                    //
                    // BUG: the target offset could be outdated since
                    // it will be modifed by the future relocations.
                    funcs.values_mut().for_each(|v| *v += offset as u16);
                } else {
                    self.update_pc(offset)?;
                }
            } else {
                funcs.insert(pc, target);
                let offset = relocate::pc(buffer, Default::default(), target as usize, true)?;
                self.update_label_pc(offset)?;
            }
        }

        relocate::funcs(funcs, buffer)?;
        Ok(())
    }

    /// Get the target of a jump.
    pub fn target(&self, jump: &Jump) -> Result<u16> {
        match jump {
            Jump::Label(label) => Ok(*label),
            Jump::Func(func) => Ok(*self.func.get(func).ok_or(Error::FuncNotFound(*func))?),
        }
    }

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
