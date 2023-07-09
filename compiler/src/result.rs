//! Zinkc result

/// Zinkc errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    BinaryReader(#[from] wasmparser::BinaryReaderError),
    #[error(transparent)]
    Codegen(#[from] zingen::Error),
    #[error("Invalid function signature")]
    InvalidFunctionSignature,
}

/// Zinkc result
pub type Result<T> = std::result::Result<T, Error>;
