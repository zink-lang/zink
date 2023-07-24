//! Jump table implementation.

use crate::{Error, Result};
use std::collections::BTreeMap;

mod pc;
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
            table.shift_pc(pc, pc)?;
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

    /// Get the target of a jump.
    pub fn target(&self, jump: &Jump) -> Result<u16> {
        match jump {
            Jump::Label(label) => Ok(*label),
            Jump::Func(func) => Ok(*self.func.get(func).ok_or(Error::FuncNotFound(*func))?),
        }
    }
}
