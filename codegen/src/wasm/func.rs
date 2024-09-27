//! Function handler
use crate::{wasm::Exports, Error, Result};
use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};
use wasmparser::{FuncType, FuncValidator, FunctionBody, ValidatorResources, WasmModuleResources};

/// Function with validator.
pub struct Function<'f> {
    /// Function validator.
    pub validator: FuncValidator<ValidatorResources>,
    /// Function body.
    pub body: FunctionBody<'f>,
}

impl Function<'_> {
    /// Get function index.
    pub fn index(&self) -> u32 {
        self.validator.index()
    }

    /// Get the function signature.
    pub fn sig(&self) -> Result<FuncType> {
        let func_index = self.validator.index();
        let sig = self
            .validator
            .resources()
            .type_of_function(func_index)
            .ok_or(Error::InvalidFunctionSignature)?
            .clone();

        Ok(sig)
    }
}

/// WASM Functions by indexes.
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
