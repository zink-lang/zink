//! Zink sdk results.

use crate::Signer;

/// Zink SDK error.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Ethers abi error.
    #[error(transparent)]
    Abi(#[from] ethers::abi::AbiError),
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

/// Zink SDK result.
pub type Result<T> = std::result::Result<T, Error>;
