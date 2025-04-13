//! Binary lookup util

use anyhow::Result;
use serde::Deserialize;
use std::{fs, path::PathBuf};

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

/// Get the name of the current package.
pub fn pkg_name() -> Result<String> {
    let manifest = fs::read_to_string(etc::find_up("Cargo.toml")?)?;
    Ok(toml::from_str::<Manifest>(&manifest)?.package.name)
}

/// Get the wasm binary of the provided name from the target directory.
pub fn wasm(name: &str) -> Result<PathBuf> {
    let target = target_dir()?;
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

    search("release").or_else(|_| search("debug"))
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
