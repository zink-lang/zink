//! Zink compiler.

pub use crate::{
    result::{Error, Result},
    wasm::WasmBuilder,
};

pub mod result;
mod wasm;

/// Target platform.
pub enum Target {
    /// Compile code to WebAssembly.
    WASM,
    /// Compile code to Ethereum Virtual Machine bytecode.
    EVM,
}

/// Zink compiler.
pub struct Compiler;
