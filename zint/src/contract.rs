//! Contract Instance

use crate::{lookup, Bytes32, Info, EVM};
use anyhow::{anyhow, Result};
use std::fs;
use zinkc::{Artifact, Compiler, Config, Constructor, InitStorage};

/// Contract instance for testing.
#[derive(Default)]
pub struct Contract {
    /// If enable dispatcher.
    pub dispatcher: bool,
    /// The artifact of the contract.
    pub artifact: Artifact,
    /// The source WASM of the contract.
    pub wasm: Vec<u8>,
    /// Bytecode constructor
    pub constructor: Constructor,
    /// Address in evm
    pub address: [u8; 20],
}

impl<T> From<T> for Contract
where
    T: AsRef<[u8]>,
{
    fn from(wasm: T) -> Self {
        crate::setup_logger();

        Self {
            wasm: wasm.as_ref().into(),
            dispatcher: true,
            ..Default::default()
        }
    }
}

impl Contract {
    /// Get the bytecode of the contract.
    pub fn bytecode(&self) -> Result<Vec<u8>> {
        let bytecode = self
            .constructor
            .finish(self.artifact.runtime_bytecode.clone().into())
            .map(|v| v.to_vec())?;

        tracing::debug!("runtime bytecode: {}", hex::encode(&bytecode));
        Ok(bytecode)
    }

    /// Preset the storage of the contract, similar with the concept `constructor`
    /// in solidity, but just in time.
    pub fn construct(&mut self, storage: InitStorage) -> Result<&mut Self> {
        self.constructor.storage(storage)?;
        Ok(self)
    }

    /// Compile WASM to EVM bytecode.
    pub fn compile(mut self) -> Result<Self> {
        let config = Config::default().dispatcher(self.dispatcher);
        let compiler = Compiler::new(config);
        self.artifact = compiler.compile(&self.wasm)?;

        // tracing::debug!("abi: {:#}", self.json_abi()?);
        tracing::debug!("bytecode: {}", hex::encode(&self.artifact.runtime_bytecode));
        Ok(self)
    }

    /// Deploy self to evm
    pub fn deploy<'e>(&mut self) -> Result<EVM<'e>> {
        let mut evm = EVM::default();
        let info = evm.deploy(&self.bytecode()?)?;

        self.address.copy_from_slice(&info.address);
        Ok(evm)
    }

    /// Load zink contract defined in the current
    /// package.
    ///
    /// NOTE: This only works if the current contract
    /// is not an example.
    pub fn current() -> Result<Self> {
        Self::search(&lookup::pkg_name()?)
    }

    /// Encode call data
    pub fn encode<Param>(&self, inputs: impl AsRef<[Param]>) -> Result<Vec<u8>>
    where
        Param: Bytes32,
    {
        let mut calldata = Vec::new();
        let mut inputs = inputs.as_ref();
        if self.dispatcher {
            if inputs.is_empty() {
                return Err(anyhow!("no selector provided"));
            }

            calldata.extend_from_slice(&zabi::selector::parse(&inputs[0].to_vec()));
            inputs = &inputs[1..];
        }

        for input in inputs {
            calldata.extend_from_slice(&input.to_bytes32());
        }

        tracing::debug!("calldata: {}", hex::encode(&calldata));
        Ok(calldata)
    }

    /// Execute the contract.
    pub fn execute<Param>(&mut self, inputs: impl AsRef<[Param]>) -> Result<Info>
    where
        Param: Bytes32,
    {
        EVM::interp(&self.artifact.runtime_bytecode, &self.encode(inputs)?)
    }

    /// Get the JSON ABI of the contract.
    pub fn json_abi(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.artifact.abi).map_err(Into::into)
    }

    /// Disable dispatcher.
    pub fn pure(mut self) -> Self {
        self.dispatcher = false;
        self
    }

    /// Search for zink contract in the target directory.
    pub fn search(name: &str) -> Result<Self> {
        crate::setup_logger();
        let wasm = lookup::wasm(name)?;
        zinkc::utils::wasm_opt(&wasm, &wasm)?;

        tracing::debug!("loading contract from {}", wasm.display());
        Ok(Self::from(fs::read(wasm)?))
    }
}
