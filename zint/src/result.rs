//! Zink sdk results.

/// Zint error.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[cfg(feature = "ethers")]
    /// Ethers abi error.
    #[error(transparent)]
    Abi(#[from] ethers::abi::AbiError),
    /// Anyhow error.
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[cfg(feature = "ethers")]
    /// Ethers contract error.
    #[error(transparent)]
    Contract(#[from] ethers::middleware::contract::ContractError<crate::api::Signer>),
    #[cfg(feature = "ethers")]
    /// Ethers wallet error.
    #[error(transparent)]
    Wallet(#[from] ethers::signers::WalletError),
}

/// Zint result.
pub type Result<T> = std::result::Result<T, Error>;
