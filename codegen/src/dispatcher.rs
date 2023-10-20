//! Code generator for EVM dispatcher.

use crate::{export::Export, DataSet, Error, Exports, Imports, JumpTable, MacroAssembler, Result};
use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};
use wasmparser::{FuncValidator, FunctionBody, Operator, ValidatorResources};

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

impl<'f> Deref for Functions<'f> {
    type Target = BTreeMap<u32, Function<'f>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'f> DerefMut for Functions<'f> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
    pub fn drain_selectors(&mut self, exports: &Exports) -> Self {
        let mut functions = Self::default();

        for index in exports.selectors() {
            if let Some(function) = self.0.remove(&index) {
                functions.0.insert(index, function);
            }
        }

        functions
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
    pub asm: MacroAssembler,
    /// Module exports
    pub exports: Exports,
    /// Module imports
    pub imports: Imports,
    /// Module data
    pub data: DataSet,
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

    /// Set data for the dispatcher.
    pub fn data(&mut self, data: DataSet) -> &mut Self {
        self.data = data;
        self
    }

    /// Query exported function from selector.
    fn query_from_selector(&self, index: u32) -> Result<u32> {
        let Export { name, kind: _ } = self.exports.get(index).ok_or(Error::InvalidSelector)?;
        let mut name = name.to_string();
        name = name.trim_end_matches("_selector").into();

        for (index, export) in self.exports.iter() {
            if export.name == name {
                return Ok(*index);
            }
        }

        Err(Error::FuncNotFound(index))
    }

    /// Emit selector to buffer
    fn emit_selector(&mut self, target: u32, selector: &Function<'_>) -> Result<()> {
        let mut reader = selector.body.get_operators_reader()?;

        // Get data offset.
        let Operator::I32Const { value: offset } = reader.read()? else {
            return Err(Error::InvalidSelector);
        };

        // Get data length.
        let Operator::I32Const { value: length } = reader.read()? else {
            return Err(Error::InvalidSelector);
        };

        // Validate zinkc helper `emit_abi`
        let Operator::Call {
            function_index: index,
        } = reader.read()?
        else {
            return Err(Error::InvalidSelector);
        };
        if !self.imports.is_emit_abi(index) {
            return Err(Error::FuncNotImported("emit_abi"));
        }

        let data = self.data.load(offset, length as usize)?;
        let _func = self.query_from_selector(target);
        tracing::debug!("selector: {:x?}", data);

        // self.asm._dup1()?;
        // self.asm.push(&data)?;
        // self.asm._eq()?;

        // JUMPI here.

        Ok(())
    }

    /// Emit compiled code to the given buffer.
    pub fn finish(mut self, selectors: Functions<'_>, table: &mut JumpTable) -> Result<Vec<u8>> {
        for (index, func) in selectors.iter() {
            self.emit_selector(*index, func)?;
        }

        table.merge(self.table, 0)?;
        Ok(self.asm.buffer().into())
    }
}
