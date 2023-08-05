//! Command publish
use anyhow::Result;
use cargo_metadata::MetadataCommand;
use clap::Parser;
use std::{
    path::{Path, PathBuf},
    process::Command,
    thread,
    time::Duration,
};

/// Publish crates.
#[derive(Debug, Parser, Clone)]
pub struct Publish {
    /// If allow dirty publish.
    #[clap(short, long, value_name = "dry-run")]
    allow_dirty: bool,

    /// If dry run.
    #[clap(short, long, value_name = "dry-run")]
    dry_run: bool,
}

impl Publish {
    /// Run publish
    pub fn run(&self, manifest: &PathBuf, packages: &[String]) -> Result<()> {
        let metadata = MetadataCommand::new()
            .no_deps()
            .manifest_path(manifest)
            .exec()?;

        for pkg in metadata.packages {
            if !packages.contains(&pkg.name) {
                continue;
            }

            if self.publish(&pkg.manifest_path).is_err() {
                // Just handle the rate limit for once.
                thread::sleep(Duration::from_secs(60 * 6));
                self.publish(pkg.manifest_path)?;
            }
        }

        Ok(())
    }

    /// Publish cargo package
    fn publish(&self, path: impl AsRef<Path>) -> Result<()> {
        let mut cargo = Command::new("cargo");
        cargo
            .arg("publish")
            .arg("--manifest-path")
            .arg(&path.as_ref());

        if self.dry_run {
            cargo.arg("--dry-run");
        }

        if self.allow_dirty {
            cargo.arg("--allow-dirty");
        }

        cargo.status()?;
        Ok(())
    }
}
