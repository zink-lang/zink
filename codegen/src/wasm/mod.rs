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
use std::collections::BTreeMap;

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
