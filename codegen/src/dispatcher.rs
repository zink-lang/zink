//! Code generator for EVM dispatcher.

use crate::{Buffer, Exports, Imports, JumpTable, Result};
use std::collections::BTreeMap;
use wasmparser::{FuncValidator, FunctionBody, ValidatorResources};

/// Function with validator.
pub struct Function<'f> {
    /// Function validator.
    pub validator: FuncValidator<ValidatorResources>,
    /// Function body.
    pub body: FunctionBody<'f>,
}

/// Functions with indexes.
#[derive(Default)]
pub struct Functions<'f>(BTreeMap<u32, Function<'f>>);

impl<'f> Functions<'f> {
    /// Add function to the list.
    pub fn add(
        &mut self,
        validator: FuncValidator<ValidatorResources>,
        function: FunctionBody<'f>,
    ) {
        self.0.insert(
            validator.index(),
            Function {
                validator,
                body: function,
            },
        );
    }

    /// Remove all selector functions
    pub fn drain_selectors(&mut self, exports: &Exports) -> Result<Self> {
        let mut functions = Self::default();

        for index in exports.selectors() {
            if let Some(function) = self.0.remove(&index) {
                functions.0.insert(index, function);
            }
        }

        Ok(functions)
    }

    /// Get all functions
    pub fn into_funcs(self) -> Vec<Function<'f>> {
        self.0.into_values().collect()
    }
}

/// Code generator for EVM dispatcher.
#[derive(Default)]
pub struct Dispatcher {
    /// Code buffer
    pub buffer: Buffer,
    /// Module exports
    pub exports: Exports,
    /// Module imports
    pub imports: Imports,
    /// Jump table
    pub table: JumpTable,
}

impl Dispatcher {
    /// Set exports for the dispatcher.
    pub fn exports(&mut self, exports: Exports) -> &mut Self {
        self.exports = exports;
        self
    }

    /// Set imports for the dispatcher.
    pub fn imports(&mut self, imports: Imports) -> &mut Self {
        self.imports = imports;
        self
    }

    /// Emit compiled code to the given buffer.
    pub fn finish(&mut self, _table: &mut JumpTable) -> Result<Vec<u8>> {
        Ok(vec![])
    }
}
