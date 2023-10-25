//! Contract Instance

use crate::{Bytes32, Info, EVM};
use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};
use wasm_opt::OptimizationOptions;
use zinkc::Compiler;

/// Cargo Package for parsing package name.
#[derive(Deserialize)]
struct Package {
    name: String,
}

/// Contract instance for testing.
#[derive(Default)]
pub struct Contract {
    /// The bytecode of the contract.
    pub bytecode: Vec<u8>,
    /// If enable dispatcher.
    pub dispatcher: bool,
    /// The source WASM of the contract.
    pub wasm: Vec<u8>,
}

impl Contract {
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

    /// Run wasm-opt on the given WASM file.
    fn wasm_opt(wasm: impl AsRef<Path>) -> Result<()> {
        OptimizationOptions::new_opt_level_4()
            .debug_info(false)
            .mvp_features_only()
            .set_converge()
            .run(&wasm, &wasm)
            .map_err(Into::into)
    }

    /// Create new contract
    pub fn new(wasm: Vec<u8>) -> Self {
        Self {
            wasm,
            dispatcher: true,
            ..Default::default()
        }
    }

    /// Disable dispatcher.
    pub fn without_dispatcher(mut self) -> Self {
        self.dispatcher = false;
        self
    }

    /// Compile WASM to EVM bytecode.
    pub fn compile(mut self) -> Result<Self> {
        self.bytecode = Compiler::default()
            .dispatcher(self.dispatcher)
            .compile(&self.wasm)?
            .to_vec();

        Ok(self)
    }

    /// Load zink contract defined in the current
    /// package.
    pub fn current() -> Result<Self> {
        let manifest = fs::read_to_string(etc::find_up("Cargo.toml")?)?;
        let name = toml::from_str::<Package>(&manifest)?.name;

        Self::search(&name)
    }

    /// Search for zink contract in the target
    /// directory.
    pub fn search(name: &str) -> Result<Self> {
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
        Self::wasm_opt(&wasm)?;

        tracing::debug!("loading contract from {}", wasm.display());
        Ok(Self::new(fs::read(wasm)?))
    }

    /// Execute the contract.
    pub fn execute(&mut self, mut inputs: &[impl Bytes32]) -> Result<Info> {
        let mut calldata = Vec::new();
        if self.dispatcher {
            if inputs.is_empty() {
                return Err(anyhow!("no selector provided"));
            }

            calldata.extend_from_slice(&zabi::selector(&inputs[0].to_vec()));
            inputs = &inputs[1..];
        }

        for input in inputs {
            calldata.extend_from_slice(&input.to_bytes32());
        }

        Ok(EVM::run(&self.bytecode, &calldata))
    }
}
