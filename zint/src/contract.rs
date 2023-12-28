//! Contract Instance

use crate::{Bytes32, Info, EVM};
use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::{fs, path::PathBuf};
use zabi::Abi;
use zinkc::{Compiler, Config};

/// Cargo Manifest for parsing package.
#[derive(Deserialize)]
struct Manifest {
    /// The package.
    pub package: Package,
}

/// Cargo Package for parsing package name.
#[derive(Deserialize)]
struct Package {
    /// Package name.
    pub name: String,
}

/// Contract instance for testing.
#[derive(Default)]
pub struct Contract {
    /// The bytecode of the contract.
    pub bytecode: Vec<u8>,
    /// The runtime bytecode of the contract.
    pub runtime_bytecode: Vec<u8>,
    /// If enable dispatcher.
    pub dispatcher: bool,
    /// If enable constructor.
    pub constructor: bool,
    /// The ABI of the contract.
    pub abi: Vec<Abi>,
    /// The source WASM of the contract.
    pub wasm: Vec<u8>,
}

impl Contract {
    /// Create new contract
    pub fn new(wasm: impl AsRef<[u8]>) -> Self {
        crate::setup_logger();

        Self {
            wasm: wasm.as_ref().into(),
            dispatcher: true,
            ..Default::default()
        }
    }

    /// Disable dispatcher.
    pub fn constructor(mut self, constructor: bool) -> Self {
        self.constructor = constructor;
        self
    }

    /// Disable dispatcher.
    pub fn without_dispatcher(mut self) -> Self {
        self.dispatcher = false;
        self
    }

    /// Get the JSON ABI of the contract.
    pub fn json_abi(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.abi).map_err(Into::into)
    }

    /// Compile WASM to EVM bytecode.
    pub fn compile(mut self) -> Result<Self> {
        let config = Config::default()
            .constructor(self.constructor)
            .dispatcher(self.dispatcher);

        let compiler = Compiler::new(config);
        let artifact = compiler.compile(&self.wasm)?;
        self.bytecode = artifact.bytecode.clone();
        self.runtime_bytecode = artifact.runtime_bytecode.clone();
        self.abi = artifact.abi.clone();

        tracing::debug!("abi: {:#}", self.json_abi()?);
        tracing::debug!("bytecode: {:?}", hex::encode(&self.bytecode));
        Ok(self)
    }

    /// Load zink contract defined in the current
    /// package.
    ///
    /// NOTE: This only works if the current contract
    /// is not an example.
    pub fn current() -> Result<Self> {
        let manifest = fs::read_to_string(etc::find_up("Cargo.toml")?)?;
        let name = toml::from_str::<Manifest>(&manifest)?.package.name;

        Self::search(&name)
    }

    /// Search for zink contract in the target
    /// directory.
    pub fn search(name: &str) -> Result<Self> {
        crate::setup_logger();

        let target = Self::target_dir()?;
        let search = |profile: &str| -> Result<PathBuf> {
            let target = target.join(profile);
            let mut wasm = target.join(name).with_extension("wasm");
            if !wasm.exists() {
                wasm = target.join("examples").join(name).with_extension("wasm");
            }

            if wasm.exists() {
                Ok(wasm)
            } else {
                Err(anyhow::anyhow!("{} not found", wasm.to_string_lossy()))
            }
        };

        let wasm = search("release").or_else(|_| search("debug"))?;
        zinkc::utils::wasm_opt(&wasm, &wasm)?;

        tracing::debug!("loading contract from {}", wasm.display());
        Ok(Self::new(fs::read(wasm)?))
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

        Ok(calldata)
    }

    /// Execute the contract.
    pub fn execute<Param>(&mut self, inputs: impl AsRef<[Param]>) -> Result<Info>
    where
        Param: Bytes32,
    {
        EVM::interp(&self.runtime_bytecode, &self.encode(inputs)?)
    }

    /// Get the current target directory.
    fn target_dir() -> Result<PathBuf> {
        cargo_metadata::MetadataCommand::new()
            .no_deps()
            .exec()
            .map_err(Into::into)
            .map(|metadata| {
                metadata
                    .target_directory
                    .join("wasm32-unknown-unknown")
                    .into()
            })
    }
}
