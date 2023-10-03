//! Command publish
use anyhow::{anyhow, Result};
use cargo_metadata::MetadataCommand;
use clap::Parser;
use crates_io::Registry;
use curl::easy::Easy;
use std::{collections::BTreeMap, path::PathBuf, process::Command};

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
        let mut cargo = Command::new("cargo");
        cargo.arg("publish").arg("-p").arg(package);

        if self.dry_run {
            cargo.arg("--dry-run");
        }

        if self.allow_dirty {
            cargo.arg("--allow-dirty");
        }

        Ok(cargo.status()?.success())
    }

    fn verify(&self, manifest: &PathBuf, packages: &[String]) -> Result<Vec<String>> {
        let mut registry = {
            let mut handle = Easy::new();
            handle.useragent("zink-lang")?;
            Registry::new_handle("https://crates.io".into(), None, handle, false)
        };

        let metadata = MetadataCommand::new()
            .no_deps()
            .manifest_path(manifest)
            .exec()?;

        let pkgs = metadata
            .packages
            .iter()
            .map(|pkg| (pkg.name.clone(), pkg.version.to_string()))
            .collect::<BTreeMap<_, _>>();

        packages
            .into_iter()
            .filter_map(|pkg| -> Option<Result<_>> {
                let Some((name, version)) = pkgs.get_key_value(pkg) else {
                    return Some(Err(anyhow!("Package {} not found in metadata", pkg)));
                };

                if let Ok((crates, _total)) = registry.search(&pkg, 1) {
                    if crates.len() == 1 && crates[0].max_version == *version {
                        println!("Package {}@{} has already been published.", name, version);
                        return None;
                    }
                }

                Some(Ok(name.clone()))
            })
            .collect::<Result<Vec<_>>>()
    }
}
