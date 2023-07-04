//! WASM Compiler

use crate::{Profile, Result};
use anyhow::anyhow;
use cargo_metadata::{Metadata, MetadataCommand, Package};
use etc::{Etc, FileSystem};
use std::{env, fs, path::PathBuf, process::Command};

/// WASM Builder
pub struct WasmBuilder {
    profile: Profile,
    metadata: Metadata,
    package: Package,
    #[allow(unused)]
    output: PathBuf,
    out_dir: PathBuf,
}

impl WasmBuilder {
    /// Create a new WASM Builder.
    pub fn new(path: impl Into<PathBuf>) -> Result<Self> {
        let mut metadata_command = MetadataCommand::new();
        let path = path.into();

        log::trace!("parsing cargo metadata from: {path:?}");
        let metadata = if path.is_dir() {
            metadata_command.current_dir(&path)
        } else {
            metadata_command.manifest_path(&path)
        }
        .exec()?;

        let manifest = Etc::from(path).find("Cargo.toml")?;
        log::trace!("expected manifest: {manifest:?}");
        let package = metadata
            .packages
            .iter()
            .find(|p| p.manifest_path.ends_with(&manifest))
            .ok_or(anyhow!("package {manifest:?} not found"))?
            .clone();

        let out_dir = env::current_dir()?;
        let output = out_dir.join(package.name.as_str());

        Ok(Self {
            profile: Profile::Debug,
            metadata,
            package,
            output,
            out_dir: env::current_dir()?,
        })
    }

    /// Set the profile.
    pub fn profile(&mut self, profile: impl Into<Profile>) -> &mut Self {
        self.profile = profile.into();
        self
    }

    /// Set the output directory.
    pub fn out_dir(&mut self, out_dir: impl Into<PathBuf>) -> &mut Self {
        self.out_dir = out_dir.into();
        self
    }

    /// Set the output file.
    pub fn output(&mut self, output: impl Into<PathBuf>) -> &mut Self {
        self.output = output.into();
        self
    }

    /// Run the WASM Builder.
    pub fn build(&self) -> Result<()> {
        self.compile()?;
        self.post()?;
        Ok(())
    }

    /// Compile project to WASM.
    fn compile(&self) -> Result<()> {
        let mut args = vec![
            "build",
            "--manifest-path",
            self.package.manifest_path.as_str(),
            "--target",
            "wasm32-unknown-unknown",
        ];

        if self.profile == Profile::Release {
            args.push("--release");
        }

        Command::new("cargo").args(&args).status()?;
        Ok(())
    }

    /// Post processing the built WASM files.
    fn post(&self) -> Result<()> {
        let target = self.metadata.target_directory.clone();
        let zink = target.join("zink").join(self.profile.as_ref());
        if !zink.exists() {
            fs::create_dir_all(&zink)?;
        }

        let src = target
            .join("wasm32-unknown-unknown")
            .join(self.profile.as_ref())
            .join(self.package.name.as_str())
            .with_extension("wasm");

        // copy the wasm file to the zink directory and the out directory.
        for dir in [self.out_dir.clone(), zink.into()] {
            let dst = dir.join(self.package.name.as_str()).with_extension("wasm");
            fs::copy(&src, dst)?;
        }

        Ok(())
    }
}
