//! Codegen results

/// Codegen error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Wasm(#[from] wasmtime_environ::WasmError),
}

pub type Result<T> = std::result::Result<T, Error>;
