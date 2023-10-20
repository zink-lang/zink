//! Exports in code generation.

use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

/// WASM export.
#[derive(Debug)]
pub struct Export {
    /// Name of the export.
    pub name: String,
}

/// WASM exports
#[derive(Debug, Default)]
pub struct Exports(BTreeMap<u32, Export>);

impl Deref for Exports {
    type Target = BTreeMap<u32, Export>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Exports {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Exports {
    /// Create a new empty exports.
    pub fn new() -> Self {
        Exports(BTreeMap::new())
    }

    /// Add an export.
    pub fn add(&mut self, index: u32, name: &str) {
        self.0.insert(index, Export { name: name.into() });
    }

    /// Get an export by index.
    pub fn get(&self, index: u32) -> Option<&Export> {
        self.0.get(&index)
    }

    /// Get all function selectors
    pub fn selectors(&self) -> Vec<u32> {
        let mut selectors = Vec::new();

        for (index, export) in self.0.iter() {
            if export.name.ends_with("_selector") {
                selectors.push(*index);
            }
        }

        selectors
    }
}
