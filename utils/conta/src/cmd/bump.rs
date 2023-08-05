//! Command bump
use crate::Config;
use anyhow::{anyhow, Result};
use cargo_metadata::{Metadata, MetadataCommand, Package};
use clap::Parser;
use semver::Version;
use std::{fs, path::PathBuf};

/// Bump versions.
#[derive(Debug, Parser, Clone)]
pub struct Bump {
    /// The path of the cargo manifest, if not provided, the
    /// current directory is used.
    #[clap(short, long)]
    manifest: Option<PathBuf>,

    /// The path of `Conta.toml`
    #[clap(short, long)]
    config: Option<PathBuf>,

    /// The version to bump.
    #[clap(short, long)]
    version: Version,
    // TODO:
    //
    // support bump major, minor, patch, pre, build.
}

impl Bump {
    /// Get the manifest path.
    pub fn manifest(&self) -> PathBuf {
        if let Some(p) = &self.manifest {
            p.into()
        } else {
            PathBuf::from("Cargo.toml")
        }
    }

    /// Parse the config from the input path.
    pub fn config(&self) -> Result<Config> {
        let path = if let Some(p) = &self.config {
            p.into()
        } else {
            PathBuf::from("Conta.toml")
        };

        toml::from_str(&fs::read_to_string(path)?).map_err(Into::into)
    }

    /// Get the metadata of the workspace.
    pub fn metadata(&self) -> Result<Metadata> {
        MetadataCommand::new()
            .manifest_path(self.manifest())
            .exec()
            .map_err(Into::into)
    }

    /// Bumps the version to the given one.
    ///
    /// NOTE: This implementation only works for workspace
    /// for now.
    pub fn run(&self) -> Result<()> {
        let metadata = self.metadata()?;
        let manifest = metadata
            .root_package()
            .ok_or(anyhow!("only supports workspace for now"))?;

        println!("{}", manifest.version);

        Ok(())
    }
}
