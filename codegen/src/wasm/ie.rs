//! Imports and exports

use crate::wasm::HostFunc;
use std::collections::BTreeMap;

impl_deref! {
    ("WASM import section", Imports, BTreeMap<u32, HostFunc>),
    ("WASM export section", Exports, BTreeMap<u32, String>)
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
