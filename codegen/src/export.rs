//! Exports in code generation.

use std::collections::BTreeMap;
use wasmparser::{Export as WasmExport, ExternalKind};

/// WASM export.
pub struct Export {
    /// Name of the export.
    pub name: String,
    /// Kind of the export.
    pub kind: ExternalKind,
}

/// WASM exports
#[derive(Default)]
pub struct Exports(BTreeMap<u32, Export>);

impl Exports {
    /// Create a new empty exports.
    pub fn new() -> Self {
        Exports(BTreeMap::new())
    }

    /// Add an export.
    pub fn add(&mut self, export: &WasmExport) {
        self.0.insert(
            export.index,
            Export {
                name: export.name.into(),
                kind: export.kind,
            },
        );
    }

    /// Get an export by index.
    pub fn get(&self, index: u32) -> Option<&Export> {
        self.0.get(&index)
    }
}
