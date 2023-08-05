//! Command bump
use crate::{Config, Sed};
use anyhow::Result;
use cargo_metadata::MetadataCommand;
use clap::Parser;
use semver::{Version, VersionReq};
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
    version: Version,

    /// Dry run the command and print the result.
    #[clap(short, long, value_name = "dry-run")]
    dry_run: bool,
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
    pub fn verify(&self, packages: &[String]) -> Result<()> {
        let metadata = MetadataCommand::new()
            .no_deps()
            .manifest_path(self.manifest())
            .exec()?;

        for package in metadata.packages.iter() {
            if !packages.contains(&package.name) {
                continue;
            }

            if package.version != self.version {
                return Err(anyhow::anyhow!(
                    "incorrect crate version {} in {}",
                    self.version,
                    package.name
                ));
            }
        }

        Ok(())
    }

    /// Bumps the version to the given one.
    ///
    /// NOTE: This implementation only works for workspace
    /// for now.
    pub fn run(&self) -> Result<()> {
        let config = self.config()?;
        let mut sed = Sed::new(self.manifest(), &config.packages)?;

        sed.set_workspace_version(&self.version)?;
        sed.set_dep_versions(&VersionReq::parse(format!("={}", self.version).as_str())?)?;

        if self.dry_run {
            println!("{}", String::from_utf8_lossy(&sed.buf));
        } else {
            sed.flush()?;
            self.verify(&config.packages)?;
        }

        Ok(())
    }
}
