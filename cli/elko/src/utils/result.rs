//! Zinkup result

/// Zinkc errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Anyhow error
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    /// Cargo metadata error
    #[error(transparent)]
    CargoMetadata(#[from] cargo_metadata::Error),
    /// Cargo package error
    #[error(transparent)]
    Etc(#[from] etc::Error),
    /// IO error
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// Zinkc result
pub type Result<T> = std::result::Result<T, Error>;
