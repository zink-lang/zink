//! Command bump
use crate::Config;
use anyhow::{anyhow, Result};
use ccli::clap::{self, Parser};
use semver::Version;
use std::{fs, path::PathBuf, str::FromStr};
use toml_edit::Document;

/// Bump versions.
#[derive(Debug, Parser, Clone)]
pub struct Bump {
    /// The version to bump.
    version: Version,

    /// Dry run the command and print the result.
    #[clap(short, long, value_name = "dry-run")]
    dry_run: bool,
}

impl Bump {
    /// Bumps the version to the given one.
    ///
    /// NOTE: This implementation only works for workspace
    /// for now.
    pub fn run(&self, manifest: &PathBuf, config: Config) -> Result<()> {
        let mut workspace = Document::from_str(&std::fs::read_to_string(manifest)?)?;
        let version = self.version.to_string();
        workspace["workspace"]["package"]["version"] = toml_edit::value(version.clone());

        if self.dry_run {
            println!("{workspace}");
            return Ok(());
        }

        let Some(deps) = workspace["workspace"]["dependencies"].as_table_mut() else {
            return Err(anyhow!(
                "Failed to parse dependencies from workspace {manifest:?}"
            ));
        };

        for package in config.packages {
            if !deps.contains_key(&package) {
                return Err(anyhow!("package {} not found", package));
            }

            deps[&package]["version"] = toml_edit::value(version.clone());
        }

        fs::write(manifest, workspace.to_string())?;
        Ok(())
    }
}
