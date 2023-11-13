//! Zink SDK.

use ethers::{
    abi::Abi,
    contract::{BaseContract, Contract, ContractFactory},
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer as _},
    types::H160,
    utils::{Anvil, AnvilInstance},
};
pub use result::Result;
use std::{str::FromStr, sync::Arc, time::Duration};

mod result;

type Signer = SignerMiddleware<Provider<Http>, LocalWallet>;

/// Zink SDK API.
///
/// TODO: support websocket.
pub struct Api {
    /// Ethers signer middleware.
    signer: Arc<Signer>,

    /// Anvil instance
    ///
    /// TODO: make this field testing only.
    anvil: Option<AnvilInstance>,
}

impl Api {
    /// Get signer from API.
    pub fn signer(&self) -> Arc<Signer> {
        self.signer.clone()
    }

    /// Create a new API instance with anvil.
    pub fn anvil() -> Result<Self> {
        let anvil = Anvil::new()
            .args(["--gas-limit", "2000000000000", "--gas-price", "0"])
            .spawn();

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

    /// Create a new contract instance.
    pub fn contract(
        &self,
        address: impl Into<H160>,
        abi: impl Into<BaseContract>,
    ) -> Result<Contract<Signer>> {
        Ok(Contract::new(
            address.into(),
            abi.into(),
            self.signer.clone(),
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::{Api, Result};
    use ethers::abi::{Abi, Function, Param, ParamType, StateMutability};

    #[ignore]
    #[allow(deprecated)]
    #[tokio::test]
    async fn deploy() -> Result<()> {
        let api = Api::anvil()?;
        let mut abi: Abi = Default::default();
        abi.functions.insert(
            "get".into(),
            vec![Function {
                name: "get".into(),
                inputs: Default::default(),
                outputs: vec![Param {
                    name: Default::default(),
                    kind: ParamType::Int(32usize),
                    internal_type: None,
                }],
                constant: None,
                state_mutability: StateMutability::View,
            }],
        );
        let factory = api.factory(
            abi,
            hex::decode("6029600b5f395f5f5f35f060246005f35f3560e01c60135b601b91636d4ce63c1490575b5f5460010190565b60005260206000f3").unwrap(),
        )?;

        let contract = factory.deploy(())?.send().await?;
        let r = contract.method::<(), i32>("get", ())?.call().await?;
        assert_eq!(r, 1);

        Ok(())
    }
}
