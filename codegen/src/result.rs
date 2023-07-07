//! Codegen results

/// Codegen error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to parse WASM with binary reader.
    #[error(transparent)]
    BinaryReader(#[from] wasmparser::BinaryReaderError),
}

/// Codegen result
pub type Result<T> = std::result::Result<T, Error>;
