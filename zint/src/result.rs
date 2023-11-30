//! Zink sdk results.

use crate::zethers::Signer;

/// Zint error.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Ethers abi error.
    #[error(transparent)]
    Abi(#[from] ethers::abi::AbiError),
    /// Anyhow error.
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    /// Ethers contract error.
    #[error(transparent)]
    Contract(#[from] ethers::middleware::contract::ContractError<Signer>),
    /// Url parser error.
    #[error(transparent)]
    Url(#[from] url::ParseError),
    /// Ethers wallet error.
    #[error(transparent)]
    Wallet(#[from] ethers::signers::WalletError),
}

/// Zint result.
pub type Result<T> = std::result::Result<T, Error>;
