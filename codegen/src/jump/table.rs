//! Jump Table

use crate::{codegen::ExtFunc, jump::Jump, Code, Error, Result};
use std::collections::BTreeMap;

/// Jump table implementation.
///
#[derive(Clone, Default, Debug)]
pub struct JumpTable {
    /// Jump table.
    pub(crate) jump: BTreeMap<u16, Jump>,
    /// Function table.
    pub(crate) func: BTreeMap<u32, u16>,
    /// Code section.
    pub(crate) code: Code,
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

    /// Register the start of the program counter
    /// of the code section.
    pub fn code_offset(&mut self, offset: u16) {
        self.code.shift(offset);
    }

    /// Register a external function.
    pub fn ext(&mut self, pc: u16, func: ExtFunc) {
        self.code.try_add_func(func.clone());
        self.jump.insert(pc, Jump::ExtFunc(func));
    }

    /// Register a label.
    pub fn label(&mut self, pc: u16, label: u16) {
        self.jump.insert(pc, Jump::Label(label));
    }

    /// Register a label at the specific PC offset
    pub fn offset(&mut self, pc: u16, offset: u16) {
        self.jump.insert(pc, Jump::Offset(offset));
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
            Jump::Offset(offset) => Ok(*offset),
            Jump::Label(label) => Ok(*label),
            Jump::Func(func) => Ok(*self.func.get(func).ok_or(Error::FuncNotFound(*func))?),
            Jump::ExtFunc(ext) => Ok(self.code.offset_of(ext).ok_or(Error::ExtFuncNotFound)?),
        }
    }
}
