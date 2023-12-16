//! Command publish

use crate::version;
use anyhow::{anyhow, Result};
use ccli::clap::{self, Parser};
use core::str::FromStr;
use std::{fs, path::PathBuf, process::Command};
use toml_edit::Document;

/// Publish crates.
#[derive(Debug, Parser, Clone)]
pub struct Publish;

impl Publish {
    /// Run publish
    pub fn run(&self, manifest: &PathBuf, packages: Vec<String>) -> Result<()> {
        let pkgs = self.verify(manifest, packages)?;

        for pkg in pkgs {
            if !self.publish(&pkg)? {
                return Err(anyhow!("Failed to publish {pkg}"));
            }
        }

        Ok(())
    }

    /// Publish cargo package
    fn publish(&self, package: &str) -> Result<bool> {
        Command::new("cargo")
            .arg("publish")
            .arg("-p")
            .arg(package)
            .arg("--allow-dirty")
            .status()
            .map(|status| status.success())
            .map_err(|err| err.into())
    }

    fn verify(&self, manifest: &PathBuf, packages: Vec<String>) -> Result<Vec<String>> {
        let mut workspace = Document::from_str(&std::fs::read_to_string(manifest)?)?;
        let version = workspace["package"]["version"]
            .as_str()
            .ok_or_else(|| anyhow!("Failed to parse version from workspace {manifest:?}"))?
            .to_string();

        let Some(deps) = workspace["worskpace"]["dependencies"].as_table_mut() else {
            return Err(anyhow!(
                "Failed to parse dependencies from workspace {manifest:?}"
            ));
        };

        let mut unpublished = vec![];
        for (key, dep) in deps.iter_mut() {
            let name = key.get();
            if !packages.contains(&name.into()) {
                continue;
            }

            if version::verify(name, &version)? {
                println!("Package {}@{} has already been published.", name, version);
                continue;
            }

            dep["version"] = toml_edit::value(version.clone());
            let name = dep["package"].as_str().unwrap_or(name);
            unpublished.push(name.into());
        }

        fs::write(manifest, workspace.to_string())?;
        Ok(unpublished)
    }
}
