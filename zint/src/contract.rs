//! Contract Instance

use anyhow::Result;
use etc::{Etc, FileSystem};
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
            .map(|metadata| metadata.target_directory.into())
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

    /// Compile WASM to EVM bytecode.
    pub fn compile(&mut self) -> Result<&mut Self> {
        self.bytecode = Compiler::default()
            .dispatcher(self.dispatcher)
            .compile(&mut self.wasm)?
            .to_vec();

        Ok(self)
    }

    /// Load zink contract defined in the current
    /// package.
    pub fn current() -> Result<Self> {
        let manifest = fs::read_to_string(etc::find_up("Cargo.toml")?)?;
        let name = toml::from_str::<Package>(&manifest)?.name;
        let wasm = Self::target_dir()?
            .join("wasm32-unknown-unknown")
            .join("release")
            .join(&name)
            .with_extension("wasm");

        Self::wasm_opt(&wasm)?;
        Ok(Self::new(fs::read(wasm)?))
    }

    /// Search for zink contract in the target
    /// directory.
    pub fn search(name: &str) -> Result<Self> {
        let release = Self::target_dir()?
            .join("wasm32-unknown-unknown")
            .join("release");

        let mut wasm = release.join(name).with_extension("wasm");
        if !wasm.exists() {
            wasm = release.join("examples").join(name).with_extension("wasm");
        }

        Self::wasm_opt(&wasm)?;
        Ok(Self::new(fs::read(wasm)?))
    }

    /// Disable dispatcher.
    pub fn without_dispatcher(mut self) -> Self {
        self.dispatcher = false;
        self
    }
}
