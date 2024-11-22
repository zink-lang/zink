//! Jump Table
//!
//! This module defines the `JumpTable` struct, which manages the jump table, function
//! table, and code section. It provides methods to register jumps, functions, and
//! labels, as well as to merge jump tables.

use crate::{codegen::ExtFunc, jump::Jump, Code, Error, Result};
use std::collections::BTreeMap;

/// Jump table implementation.
#[derive(Clone, Default, Debug)]
pub struct JumpTable {
    /// Jump table mapping program counters to jump types.
    pub(crate) jump: BTreeMap<u16, Jump>,
    /// Function table mapping function indices to program counters.
    pub(crate) func: BTreeMap<u32, u16>,
    /// Code section associated with the jump table.
    pub(crate) code: Code,
}

impl JumpTable {
    /// Registers a function in the jump table.
    ///
    /// This function associates a program counter with a function.
    pub fn call(&mut self, pc: u16, func: u32) {
        self.jump.insert(pc, Jump::Func(func));
    }

    /// Registers a program counter to the function table.
    ///
    /// This function associates a function with a specific offset in the function table.
    pub fn call_offset(&mut self, func: u32, offset: u16) -> Result<()> {
        if self.func.insert(func, offset).is_some() {
            return Err(Error::DuplicateFunc(func));
        }

        Ok(())
    }

    /// Registers the start of the program counter for the code section.
    pub fn code_offset(&mut self, offset: u16) {
        self.code.shift(offset);
    }

    /// Registers an external function in the jump table.
    pub fn ext(&mut self, pc: u16, func: ExtFunc) {
        self.code.try_add_func(func.clone());
        self.jump.insert(pc, Jump::ExtFunc(func));
    }

    /// Registers a label in the jump table.
    pub fn label(&mut self, pc: u16, label: u16) {
        self.jump.insert(pc, Jump::Label(label));
    }

    /// Registers a label at a specific program counter offset.
    pub fn offset(&mut self, pc: u16, offset: u16) {
        self.jump.insert(pc, Jump::Offset(offset));
    }

    /// Merges another jump table into this one.
    ///
    /// This function updates the program counters of the target jump table and
    /// handles any potential duplicates.
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
}
