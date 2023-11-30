//! Zink SDK.

use crate::Result;
use ethers::{
    abi::Abi,
    contract::ContractFactory,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer as _},
    utils::{Anvil, AnvilInstance},
};
use std::{str::FromStr, sync::Arc, time::Duration};

/// Ethers signer middleware.
pub type Signer = SignerMiddleware<Provider<Http>, LocalWallet>;

/// Ethers API.
pub struct Ethers {
    /// Ethers signer middleware.
    signer: Arc<Signer>,

    /// Anvil instance
    ///
    /// TODO: make this field testing only.
    anvil: Option<AnvilInstance>,
}

impl Ethers {
    /// Get signer from API.
    pub fn signer(&self) -> Arc<Signer> {
        self.signer.clone()
    }

    /// Create a new API instance with anvil.
    pub fn anvil() -> Result<Self> {
        let anvil = Anvil::new().spawn();

        let mut api = Self::new(
            &anvil.endpoint(),
            &hex::encode(anvil.keys()[0].clone().to_bytes()),
            Some(anvil.chain_id()),
        )?;

        api.anvil = Some(anvil);
        Ok(api)
    }

    /// Create a new API instance with http.
    pub fn new(uri: &str, key: &str, chain_id: Option<u64>) -> Result<Self> {
        let wallet = {
            let wallet = LocalWallet::from_str(key)?;
            if let Some(chain_id) = chain_id {
                wallet.with_chain_id(chain_id)
            } else {
                wallet
            }
        };

        let provider =
            Provider::<Http>::try_from(uri.to_string())?.interval(Duration::from_millis(10));
        let signer = Arc::new(SignerMiddleware::new(provider, wallet));

        Ok(Self {
            anvil: None,
            signer,
        })
    }

    /// Create a new contract factory.
    pub fn factory(
        &self,
        contract: Abi,
        bytecode: impl AsRef<[u8]>,
    ) -> Result<ContractFactory<Signer>> {
        Ok(ContractFactory::new(
            contract,
            Vec::<u8>::from(bytecode.as_ref()).into(),
            self.signer.clone(),
        ))
    }
}
