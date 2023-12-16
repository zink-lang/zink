//! Command publish

use crate::version;
use anyhow::{anyhow, Result};
use ccli::clap::{self, Parser};
use core::str::FromStr;
use std::{path::PathBuf, process::Command};
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
        let workspace = Document::from_str(&std::fs::read_to_string(manifest)?)?;
        let version = workspace["workspace"]["package"]["version"]
            .as_str()
            .ok_or_else(|| anyhow!("Failed to parse version from workspace {manifest:?}"))?;

        let Some(deps) = workspace["workspace"]["dependencies"].as_table() else {
            return Err(anyhow!(
                "Failed to parse dependencies from workspace {manifest:?}"
            ));
        };

        let mut unpublished = vec![];
        for package in packages {
            if !deps.contains_key(&package) {
                continue;
            }

            let name = deps[&package]
                .get("package")
                .and_then(|p| p.as_str())
                .unwrap_or(&package);

            if version::verify(name, version)? {
                println!("Package {name}@{version} has already been published.");
                continue;
            }

            unpublished.push(name.into());
        }

        Ok(unpublished)
    }
}
