//! Dummy WASM executor for parsing ABI
#![allow(unused)]

use anyhow::Result;
use wasmtime::{Engine, Linker, Module};

/// Dummy WASM executor.
pub struct Executor {}

impl Executor {
    // /// New dummy executor
    // pub fn new() -> Result<()> {
    //     let engine = Engine::default();
    //
    //     Ok(())
    // }
}
