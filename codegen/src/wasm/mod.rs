//! WASM related primitives.

mod abi;
mod data;
mod func;
mod host;

pub use self::{
    abi::{ToLSBytes, Type},
    data::Data,
    func::{Function, Functions},
    host::HostFunc,
};
use crate::{Error, Result};
use std::collections::BTreeMap;
use wasmparser::Operator;
use zabi::Abi;

macro_rules! impl_deref {
    ($doc:literal, $name:ident, $target:ty) => {
        #[derive(Clone, Debug, Default)]
        #[doc = concat!(" ", $doc)]
        pub struct $name($target);

        impl core::ops::Deref for $name {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl core::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
    ($(($doc:literal, $name:ident, $target:ty)),*) => {
        $( impl_deref!($doc, $name, $target); )*
    };
}

impl_deref! {
    ("WASM import section", Imports, BTreeMap<u32, HostFunc>),
    ("WASM export section", Exports, BTreeMap<u32, String>)
}

/// A struct that holds the environment wasm module.
#[derive(Clone, Debug, Default)]
pub struct Env {
    /// WASM imports
    pub imports: Imports,
    /// WASM exports
    pub exports: Exports,
    /// WASM data slots
    pub data: Data,
}

impl Env {
    /// Load abis from functions
    pub fn load_abis(&self, funs: &Functions<'_>) -> Result<Vec<Abi>> {
        let mut abis: Vec<_> = Default::default();
        for (_, fun) in funs.iter() {
            abis.push(self.load_abi(fun)?);
        }

        Ok(abis)
    }

    /// Load abi from function
    pub fn load_abi(&self, fun: &Function<'_>) -> Result<Abi> {
        let mut reader = fun.body.get_operators_reader()?;

        let Operator::I32Const { value: offset } = reader.read()? else {
            return Err(Error::InvalidSelector);
        };
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
            return Err(Error::FuncNotImported("emit_abi".into()));
        }

        let abi = self.data.load(offset, length as usize)?;
        Abi::from_hex(String::from_utf8_lossy(&abi)).map_err(Into::into)
    }

    /// Query exported function from selector.
    pub fn query_func(&self, name: &str) -> Result<u32> {
        for (index, export) in self.exports.iter() {
            if export == name {
                return Ok(*index);
            }
        }

        Err(Error::FuncNotImported(name.into()))
    }
}

impl Imports {
    /// If the function is `emit_abi`.
    pub fn is_emit_abi(&self, index: u32) -> bool {
        self.get(&index) == Some(&HostFunc::EmitABI)
    }
}

impl Exports {
    /// Get all function selectors
    pub fn selectors(&self) -> Vec<u32> {
        self.iter()
            .filter_map(|(index, export)| {
                if export.ends_with("_selector") {
                    Some(*index)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}
