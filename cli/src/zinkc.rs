//! Zinkc cli
use crate::App;
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

/// Command line interface for the zink compiler.
#[derive(Debug, Parser)]
#[command(version)]
pub struct Zinkc {
    /// The path to the wasm file or the rust project directory.
    input: PathBuf,
    /// Write output to <filename>
    #[clap(short, long, value_name = "filename")]
    output: Option<PathBuf>,
    /// Write output to compiler-chosen filename in <dir>
    #[clap(long, value_name = "dir")]
    out_dir: Option<PathBuf>,
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

impl App for Zinkc {
    fn verbose(&self) -> u8 {
        self.verbose
    }

    fn run(&self) -> Result<()> {
        Ok(())
    }
}
