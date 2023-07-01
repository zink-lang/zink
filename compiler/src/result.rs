//! Zinkc result

/// Zinkc errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    CargoMetadata(#[from] cargo_metadata::Error),
    #[error(transparent)]
    Etc(#[from] etc::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// Zinkc result
pub type Result<T> = std::result::Result<T, Error>;
