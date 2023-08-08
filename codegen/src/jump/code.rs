//! Table for the code section.

use opcodes::ShangHai as OpCode;
use std::collections::HashMap;

/// Code in code section.
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Func {
    /// Run select.
    Select,
}

impl Func {
    /// Get the bytecode of the function.
    pub fn bytecode(&self) -> Vec<u8> {
        match self {
            Self::Select => [
                OpCode::JUMPDEST,
                OpCode::POP,
                OpCode::PUSH1,
                OpCode::Data(0x06),
                OpCode::ADD,
                OpCode::JUMP,
            ],
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
    /// Function table.
    funcs: HashMap<Func, usize>,
}

impl Code {
    /// Create a new code section.
    pub fn new() -> Self {
        Self {
            offset: 0,
            funcs: HashMap::new(),
        }
    }

    /// Get the functions in the code section.
    pub fn funcs(&self) -> Vec<Func> {
        self.funcs.keys().cloned().collect()
    }

    /// Shift the code section.
    pub fn shift(&mut self, offset: u16) {
        tracing::debug!("shift code section by 0x{:x} bytes.", offset);
        let offset = offset as usize;
        self.offset += offset;
        self.funcs.values_mut().for_each(|pc| *pc += offset);
    }

    /// Add a function to the code section.
    pub fn try_add_func(&mut self, func: Func) {
        if self.funcs.contains_key(&func) {
            return;
        }

        let bytecode = func.bytecode();
        let len = bytecode.len();
        self.funcs.insert(func, self.offset);
        self.offset += len;
    }

    /// Get the current offset of the code section.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Get the offset of a function.
    pub fn offset_of(&self, func: &Func) -> Option<u16> {
        self.funcs.get(func).and_then(|i| (*i).try_into().ok())
    }

    /// Get the bytecode of the code section.
    pub fn finish(&self) -> Vec<u8> {
        let mut code = Vec::new();
        for func in self.funcs.keys() {
            code.extend(func.bytecode());
        }
        code
    }
}
