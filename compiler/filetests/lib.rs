#![deny(missing_docs)]
//! Zink filetests

use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

include!(concat!(env!("OUT_DIR"), "/tests.rs"));

/// A wat test
#[derive(Clone)]
pub struct Test {
    /// The module name
    pub module: &'static str,
    /// The test name
    pub name: &'static str,
    /// The test source
    pub wasm: Vec<u8>,
}

/// A collection of wasm tests
pub struct Tests(HashMap<(&'static str, &'static str), Vec<u8>>);

impl Tests {
    /// Load test from module and name.
    pub fn load(&self, module: &str, name: &str) -> Vec<u8> {
        self.0.get(&(module, name)).unwrap().clone()
    }
}

impl Deref for Tests {
    type Target = HashMap<(&'static str, &'static str), Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Tests {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
fn run(tests: &[Test]) {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .without_time()
        .compact()
        .try_init()
        .ok();

    for Test { module, name, wasm } in tests {
        tracing::info!("Compiling {}/{}", module, name);
        zinkc::Compiler::default()
            .compile(&wasm)
            .expect("Failed to compile {module}::{name}");
    }
}

#[test]
fn examples() {
    run(&Tests::examples())
}

#[test]
fn wat() {
    run(&Tests::wat_files())
}
