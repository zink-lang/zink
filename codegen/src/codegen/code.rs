//! Table for the code section.

use indexmap::IndexMap;

/// Code section for EVM.
#[derive(Clone, Default, Debug)]
pub struct Code {
    /// The offset of the code section
    offset: usize,
    /// Function table.
    funcs: IndexMap<ExtFunc, usize>,
}

impl Code {
    /// Create a new code section.
    pub fn new() -> Self {
        Self {
            offset: 0,
            funcs: Default::default(),
        }
    }

    /// Get the functions in the code section.
    pub fn funcs(&self) -> Vec<ExtFunc> {
        self.funcs.keys().cloned().collect()
    }

    /// Shift the code section.
    pub fn shift(&mut self, offset: u16) {
        tracing::trace!("shift code section by 0x{:x} bytes.", offset);
        let offset = offset as usize;
        self.offset += offset;
        self.funcs.values_mut().for_each(|pc| *pc += offset);
    }

    /// Add a function to the code section.
    pub fn try_add_func(&mut self, func: ExtFunc) {
        if self.funcs.contains_key(&func) {
            return;
        }

        let bytecode = func.bytecode.clone();
        let len = bytecode.len();
        self.funcs.insert(func, self.offset);
        self.offset += len;
    }

    /// Get the current offset of the code section.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Get the offset of a function.
    pub fn offset_of(&self, func: &ExtFunc) -> Option<u16> {
        self.funcs.get(func).and_then(|i| (*i).try_into().ok())
    }

    /// Get the bytecode of the code section.
    pub fn finish(&self) -> Vec<u8> {
        let mut code = Vec::new();
        for func in self.funcs.keys() {
            tracing::trace!("add function to code section: {:?}", func);
            code.extend(func.bytecode.clone());
        }
        code
    }
}

/// External function in code section.
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct ExtFunc {
    /// Stack input.
    pub stack_out: u8,
    /// Stack output.
    pub stack_in: u8,
    /// The bytecode of the external function.
    pub bytecode: Vec<u8>,
}
