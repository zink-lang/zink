//! Table for the code section.

use opcodes::ShangHai as OpCode;
use std::collections::BTreeMap;

/// Code in code section.
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum Func {
    /// Run drop.
    Drop,
}

impl Func {
    /// Get the bytecode of the function.
    pub fn bytecode(&self) -> Vec<u8> {
        match self {
            Self::Drop => [OpCode::POP],
        }
        .into_iter()
        .map(|op| op.into())
        .collect()
    }
}

/// Code section for EVM.
#[derive(Default, Debug)]
pub struct Code {
    offset: usize,
    funcs: BTreeMap<Func, usize>,
}

impl Code {
    /// Create a new code section.
    pub fn new() -> Self {
        Self {
            offset: 0,
            funcs: BTreeMap::new(),
        }
    }

    /// Shift the code section.
    pub fn shift(&mut self, offset: u16) {
        let offset = offset as usize;
        self.offset += offset;
        self.funcs
            .values_mut()
            .for_each(|offset| *offset += *offset);
    }

    /// Add a function to the code section.
    pub fn add_func(&mut self, func: Func) {
        let bytecode = func.bytecode();
        let len = bytecode.len();
        self.offset += len;
        self.funcs.insert(func, self.offset);
    }

    /// Get the offset of a function.
    pub fn offset_of(&self, func: &Func) -> Option<u16> {
        self.funcs.get(func).and_then(|i| (*i).try_into().ok())
    }
}
