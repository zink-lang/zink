use crate::utils::{find_up, FoundryConfig};
use crate::{lookup, Bytes32, Info, EVM};
use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use zinkc::{Artifact, Compiler, Config, Constructor, InitStorage};

/// Represents the bytecode object in Foundry output
#[derive(Deserialize)]
struct BytecodeObject {
    object: String,
}

/// Represents a Foundry output JSON file (e.g., out/Storage.sol/Storage.json)
#[derive(Deserialize)]
pub struct FoundryOutput {
    bytecode: BytecodeObject,
}

impl FoundryOutput {
    /// Get the bytecode as a string
    pub fn bytecode(&self) -> &str {
        &self.bytecode.object
    }
}

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
    /// Locate Foundry outputs and return a list of (contract_name, abi_path, bytecode)
    pub fn find_foundry_outputs() -> Result<Vec<(String, PathBuf, Vec<u8>)>> {
        // Find foundry.toml
        let foundry_toml_path = find_up("foundry.toml")?;
        let foundry_toml_content =
            fs::read_to_string(&foundry_toml_path).context("Failed to read foundry.toml")?;
        let foundry_config: FoundryConfig =
            toml::from_str(&foundry_toml_content).context("Failed to parse foundry.toml")?;

        // Determine the output directory
        let out_dir = foundry_config
            .profile
            .default
            .out
            .unwrap_or_else(|| "out".to_string());
        let out_path = foundry_toml_path.parent().unwrap().join(&out_dir);

        let mut outputs = Vec::new();
        for entry in fs::read_dir(&out_path).context("Failed to read Foundry out directory")? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // Skip the build-info directory
                if path.file_name().and_then(|s| s.to_str()) == Some("build-info") {
                    continue;
                }
                // Look for .json files in subdirectories (e.g., out/Storage.sol/)
                for sub_entry in fs::read_dir(&path)? {
                    let sub_entry = sub_entry.context("Failed to read directory entry")?;
                    let sub_path = sub_entry.path();
                    if sub_path.extension().and_then(|s| s.to_str()) == Some("json") {
                    let file_name = sub_path
                        .file_name()
                        .context("Failed to get file name")?
                        .to_str()
                        .context("File name is not valid UTF-8")?;
                       let Some(contract_name) = file_name.strip_suffix(".json") else {
                          continue;
                        };
                            // Only process files where the contract name (before .json) looks like a contract name
                            if contract_name
                                .chars()
                                .all(|c| c.is_alphanumeric() || c == '_')
                            {
                                let content = fs::read_to_string(&sub_path)?;
                                let output: FoundryOutput =  serde_json::from_str(&content).context(format!("Failed to parse JSON for {file_name}"))?;
                                let bytecode =
                                    hex::decode(output.bytecode().trim_start_matches("0x"))
                                        .context("Failed to decode bytecode")?;
                                outputs.push((contract_name.to_string(), sub_path, bytecode));
                            }
                    }
                }
            }
        }

        Ok(outputs)
    }

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
        // TODO(g4tianx): `Contract::search` to fail properly
        // when the contract file isn’t found
        crate::setup_logger();
        let wasm = lookup::wasm(name)?;
        zinkc::utils::wasm_opt(&wasm, &wasm)?;

        tracing::debug!("loading contract from {}", wasm.display());
        Ok(Self::from(fs::read(wasm)?))
    }
}
