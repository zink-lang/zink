//! Zinkc cli
use crate::utils::{Profile, WasmBuilder};
use anyhow::{anyhow, Result};
use clap::Parser;
use etc::{Etc, FileSystem};
use std::path::PathBuf;

/// Build contract
#[derive(Debug, Parser)]
#[command(name = "build", version)]
pub struct Build {
    /// The path to the wasm file or the rust project directory.
    /// ( only support cargo project as input for now )
    ///
    /// TODO: Support wasm file as input.
    pub input: PathBuf,
    /// Write output to <filename>
    #[clap(short, long, value_name = "filename")]
    pub output: Option<PathBuf>,
    /// Write output to compiler-chosen filename in <dir>
    #[clap(long, value_name = "dir")]
    pub out_dir: Option<PathBuf>,
    /// Optimize with default optimizations
    #[clap(long)]
    pub release: bool,
}

impl Build {
    /// Run build
    pub fn run(&self) -> Result<()> {
        let profile = if self.release {
            Profile::Release
        } else {
            Profile::Debug
        };

        // Get and check the input.
        let input = &self.input;
        {
            if Etc::new(input)?.find("Cargo.toml").is_err() {
                return Ok(());
            }

            if !input.is_dir() {
                return Err(anyhow!(
                    "Only support rust project directory as input for now"
                ));
            }
        }

        // Build the wasm.
        let mut builder = WasmBuilder::new(input)?;
        {
            if let Some(out_dir) = self.out_dir.clone() {
                builder.out_dir(out_dir);
            }

            if let Some(output) = self.output.clone() {
                builder.output(output);
            }
        }

        builder.profile(profile).build()?;
        Ok(())
    }
}
