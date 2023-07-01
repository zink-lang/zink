//! Zink compiler.

pub use crate::{
    result::{Error, Result},
    wasm::WasmBuilder,
};

pub mod result;
mod wasm;

/// Compliation profile.
#[derive(PartialEq, Eq)]
pub enum Profile {
    Debug,
    Release,
}

impl From<&str> for Profile {
    fn from(profile: &str) -> Self {
        match profile {
            "release" | "production" => Profile::Release,
            _ => Profile::Debug,
        }
    }
}

impl AsRef<str> for Profile {
    fn as_ref(&self) -> &str {
        match self {
            Profile::Debug => "debug",
            Profile::Release => "release",
        }
    }
}

/// Target platform.
pub enum Target {
    /// Compile code to WebAssembly.
    WASM,
    /// Compile code to Ethereum Virtual Machine bytecode.
    EVM,
}

/// Zink compiler.
pub struct Compiler;
