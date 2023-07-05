//! Codegen results

/// Codegen error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Wasmtime environment error
    #[error(transparent)]
    Wasm(#[from] wasmtime_environ::WasmError),
}

/// Codegen result
pub type Result<T> = std::result::Result<T, Error>;
