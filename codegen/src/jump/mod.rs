//! Jump table implementation.

use crate::{Error, Func, Result};
pub use code::Code;
use std::collections::BTreeMap;

mod code;
mod pc;
mod relocate;

/// Jump types
#[derive(Clone, Copy, Debug)]
pub enum Jump {
    /// Jump to the given label, the label here is the original
    /// program counter.
    Label(u16),
    /// Jump to function.
    Func(u32),
    /// External function.
    ExtFunc(Func),
}

impl Jump {
    /// If the target is a label.
    pub fn is_label(&self) -> bool {
        matches!(self, Jump::Label(_))
    }
}

/// Jump table implementation.
#[derive(Default, Debug)]
pub struct JumpTable {
    /// Jump table.
    jump: BTreeMap<u16, Jump>,
    /// Function table.
    func: BTreeMap<u32, u16>,
    /// Code section.
    code: Code,
}

impl JumpTable {
    /// Register a function.
    pub fn call(&mut self, pc: u16, func: u32) {
        self.jump.insert(pc, Jump::Func(func));
    }

    /// Register program counter to the function table.
    pub fn call_offset(&mut self, func: u32, offset: u16) -> Result<()> {
        if self.func.insert(func, offset).is_some() {
            return Err(Error::DuplicateFunc(func));
        }

        Ok(())
    }

    /// Register program counter for code section.
    pub fn code_offset(&mut self, offset: u16) {
        self.code.shift(offset);
    }

    /// Register a external function.
    pub fn ext(&mut self, pc: u16, func: Func) {
        tracing::debug!("register external function: {:?}", func);
        self.code.try_add_func(func);
        self.jump.insert(pc, Jump::ExtFunc(func));
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
            table.shift_pc(0, pc)?;
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

        for func in table.code.funcs() {
            self.code.try_add_func(func);
        }

        Ok(())
    }

    /// Get the target of a jump.
    pub fn target(&mut self, jump: &Jump) -> Result<u16> {
        match jump {
            Jump::Label(label) => Ok(*label),
            Jump::Func(func) => Ok(*self.func.get(func).ok_or(Error::FuncNotFound(*func))?),
            Jump::ExtFunc(ext) => Ok(self.code.offset_of(ext).ok_or(Error::ExtNotFound(*ext))?),
        }
    }
}
