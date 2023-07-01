//! WASM Compiler

use crate::Result;
use anyhow::anyhow;
use cargo_metadata::{Metadata, MetadataCommand, Package};
use etc::{Etc, FileSystem};
use std::{fs, path::PathBuf};

/// WASM Compiler Profile
#[derive(PartialEq, Eq)]
pub enum Profile {
    Debug,
    Release,
}

impl From<&str> for Profile {
    fn from(profile: &str) -> Self {
        match profile.as_ref() {
            "release" | "production" => Profile::Release,
            "debug" | _ => Profile::Debug,
        }
    }
}

impl AsRef<str> for Profile {
    fn as_ref(&self) -> &str {
        match self {
            Profile::Debug => "debug",
            Profile::Release => "release",
        }
    }
}

/// WASM Builder
pub struct WasmBuilder {
    profile: Profile,
    metadata: Metadata,
    package: Package,
}

impl WasmBuilder {
    /// Create a new WASM Builder.
    pub fn new(path: impl Into<PathBuf>, profile: impl AsRef<str>) -> Result<Self> {
        let mut metadata_command = MetadataCommand::new();
        let path = path.into();
        let metadata = if path.is_dir() {
            metadata_command.current_dir(&path)
        } else {
            metadata_command.manifest_path(&path)
        }
        .exec()?;

        let manifest = Etc::from(path).find("Cargo.toml")?;
        let package = metadata
            .packages
            .iter()
            .find(|p| p.manifest_path == manifest)
            .ok_or(anyhow!("package not found"))?
            .clone();

        Ok(Self {
            profile: profile.as_ref().into(),
            metadata,
            package,
        })
    }

    /// Run the WASM Builder.
    pub fn run(&self) -> Result<()> {
        self.build()?;
        self.post()?;
        Ok(())
    }

    /// Compile project to WASM.
    fn build(&self) -> Result<()> {
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

        let meta = MetadataCommand::new();
        meta.cargo_command().args(&args).status()?;

        Ok(())
    }

    /// Post processing the built WASM files
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
        let dst = zink.join(self.package.name.as_str()).with_extension("wasm");

        fs::copy(src, dst)?;
        Ok(())
    }
}
