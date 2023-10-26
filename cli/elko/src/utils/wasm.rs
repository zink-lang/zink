//! WASM Builder

use crate::utils::{Profile, Result};
use anyhow::anyhow;
use cargo_metadata::{Metadata, MetadataCommand, Package};
use etc::{Etc, FileSystem};
use std::{fs, path::PathBuf, process::Command};
use wasm_opt::OptimizationOptions;

/// WASM Builder
pub struct WasmBuilder {
    profile: Profile,
    metadata: Metadata,
    package: Package,
    output: Option<PathBuf>,
    out_dir: Option<PathBuf>,
}

impl WasmBuilder {
    /// Create a new WASM Builder.
    pub fn new(path: impl Into<PathBuf>) -> Result<Self> {
        let mut metadata_command = MetadataCommand::new();
        let path = path.into();

        tracing::trace!("parsing cargo metadata from: {path:?}");
        let metadata = if path.is_dir() {
            metadata_command.current_dir(&path)
        } else {
            metadata_command.manifest_path(&path)
        }
        .exec()?;

        let manifest = Etc::from(path).find("Cargo.toml")?;
        tracing::trace!("expected manifest: {manifest:?}");
        let package = metadata
            .packages
            .iter()
            .find(|p| p.manifest_path.ends_with(&manifest))
            .ok_or(anyhow!("package {manifest:?} not found"))?
            .clone();

        Ok(Self {
            profile: Profile::Debug,
            metadata,
            package,
            output: None,
            out_dir: None,
        })
    }

    /// Get the profile.
    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    /// Set the profile.
    pub fn with_profile(&mut self, profile: impl Into<Profile>) -> &mut Self {
        self.profile = profile.into();
        self
    }

    /// Get the output filename.
    pub fn output(&self) -> Result<PathBuf> {
        let out_dir = self.out_dir()?;
        let output = if let Some(output) = self.output.as_ref() {
            output.into()
        } else {
            out_dir
                .join(self.package.name.as_str())
                .with_extension("wasm")
        };

        Ok(output)
    }

    /// Set the output filename.
    pub fn with_output(&mut self, output: impl Into<PathBuf>) -> &mut Self {
        self.output = Some(output.into());
        self
    }

    /// Get the output directory.
    pub fn out_dir(&self) -> Result<PathBuf> {
        let out_dir: PathBuf = if let Some(out_dir) = self.out_dir.as_ref() {
            out_dir.into()
        } else {
            let out_dir = self
                .metadata
                .target_directory
                .join("zink")
                .join(self.profile.as_ref());
            if !out_dir.exists() {
                fs::create_dir_all(&out_dir)?;
            }
            out_dir.into()
        };

        Ok(out_dir)
    }

    /// Set the output directory.
    pub fn with_out_dir(&mut self, out_dir: impl Into<PathBuf>) -> &mut Self {
        self.out_dir = Some(out_dir.into());
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
        let src = self
            .metadata
            .target_directory
            .join("wasm32-unknown-unknown")
            .join(self.profile.as_ref())
            .join(self.package.name.as_str())
            .with_extension("wasm");

        // run the wasm optimizer
        OptimizationOptions::new_opt_level_4()
            .debug_info(false)
            .mvp_features_only()
            .set_converge()
            .run(src, self.output()?)?;

        Ok(())
    }
}
