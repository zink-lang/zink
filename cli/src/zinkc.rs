//! Zinkc cli
use crate::App;
use anyhow::{anyhow, Result};
use clap::Parser;
use etc::{Etc, FileSystem};
use std::path::PathBuf;
use zinkc::WasmBuilder;

/// Command line interface for the zink compiler.
#[derive(Debug, Parser)]
#[command(name = "zinkc", version)]
pub struct Zinkc {
    /// The path to the wasm file or the rust project directory.
    /// ( only support cargo project as input for now )
    ///
    /// TODO: Support wasm file as input.
    input: PathBuf,
    /// Write output to <filename>
    #[clap(short, long, value_name = "filename")]
    output: Option<PathBuf>,
    /// Write output to compiler-chosen filename in <dir>
    #[clap(long, value_name = "dir")]
    out_dir: Option<PathBuf>,
    /// Optimize with default optimizations
    #[clap(long)]
    release: bool,
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

impl App for Zinkc {
    fn verbose(&self) -> u8 {
        self.verbose
    }

    fn run(&self) -> Result<()> {
        let profile = if self.release {
            zinkc::Profile::Release
        } else {
            zinkc::Profile::Debug
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
