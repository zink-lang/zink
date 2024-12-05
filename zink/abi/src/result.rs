//! Abi results
#![cfg(feature = "hex")]

/// ABI error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to encode or decode with postcard.
    #[error(transparent)]
    Postcard(#[from] postcard::Error),
    /// Failed to decode from hex.
    #[error(transparent)]
    Hex(#[from] hex::FromHexError),
}

/// ABI result
pub type Result<T> = std::result::Result<T, Error>;
