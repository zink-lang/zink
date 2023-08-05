//! Command publish
use anyhow::Result;
use cargo_metadata::MetadataCommand;
use clap::Parser;
use std::{path::PathBuf, process::Command, thread, time::Duration};

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
        self.verify(manifest, packages)?;

        for pkg in packages {
            if !self.publish(&pkg)? {
                // Just handle the rate limit for once.
                thread::sleep(Duration::from_secs(60 * 6));
                self.publish(pkg)?;
            }
        }

        Ok(())
    }

    /// Publish cargo package
    fn publish(&self, package: &str) -> Result<bool> {
        let mut cargo = Command::new("cargo");
        cargo.arg("publish").arg("-p").arg(&package);

        if self.dry_run {
            cargo.arg("--dry-run");
        }

        if self.allow_dirty {
            cargo.arg("--allow-dirty");
        }

        Ok(cargo.status()?.success())
    }

    fn verify(&self, manifest: &PathBuf, packages: &[String]) -> Result<()> {
        let metadata = MetadataCommand::new()
            .no_deps()
            .manifest_path(manifest)
            .exec()?;

        let pkgs = metadata
            .packages
            .iter()
            .map(|pkg| pkg.name.clone())
            .collect::<Vec<_>>();

        for pkg in packages {
            if !pkgs.contains(pkg) {
                anyhow::bail!("Package {} not found in metadata", pkg);
            }
        }

        Ok(())
    }
}
