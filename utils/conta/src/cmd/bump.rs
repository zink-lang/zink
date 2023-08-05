//! Command bump
use crate::{Config, Sed};
use anyhow::Result;
use cargo_metadata::MetadataCommand;
use clap::Parser;
use semver::{Version, VersionReq};
use std::path::PathBuf;

/// Bump versions.
#[derive(Debug, Parser, Clone)]
pub struct Bump {
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
    /// Bumps the version to the given one.
    ///
    /// NOTE: This implementation only works for workspace
    /// for now.
    pub fn run(&self, manifest: &PathBuf, config: Config) -> Result<()> {
        let mut sed = Sed::new(manifest)?;

        sed.set_workspace_version(&self.version)?;
        let version_req = format!("={}", self.version);
        sed.set_dep_versions(&VersionReq::parse(&version_req)?, &config.packages)?;

        if self.dry_run {
            println!("{}", String::from_utf8_lossy(&sed.buf));
        } else {
            sed.flush()?;
            self.verify(manifest, &config.packages)?;
        }

        Ok(())
    }

    /// Get the metadata of the workspace.
    pub fn verify(&self, manifest: &PathBuf, packages: &[String]) -> Result<()> {
        let metadata = MetadataCommand::new()
            .no_deps()
            .manifest_path(manifest)
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
}
