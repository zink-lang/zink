//! Zink Compiler
#![deny(missing_docs)]

use clap::Parser;
use color_eyre::Result;
use zinkup::{App, Compile};

/// Zink's package manager
#[derive(Debug, Parser)]
#[command(name = "zinkc", version)]
pub struct Zinkc {
    /// The entry of the zinkc compiler.
    #[command(flatten)]
    pub compile: Compile,
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

impl App for Zinkc {
    fn verbose(&self) -> u8 {
        self.verbose
    }

    fn run(&self) -> anyhow::Result<()> {
        self.compile.run()
    }
}

/// The main function.
fn main() -> Result<()> {
    Zinkc::start()
}
