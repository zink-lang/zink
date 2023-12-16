//! Zink compiler.
#![deny(missing_docs)]
#![cfg(feature = "cli")]

use ccli::{clap, App, Parser, Result};
use zinkc::cli::Compile;

/// The Zink Compiler.
#[derive(Debug, Parser)]
#[command(name = "zinkc", version, arg_required_else_help(true))]
pub struct Zinkc {
    #[clap(flatten)]
    command: Compile,
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

impl App for Zinkc {
    fn verbose(&self) -> u8 {
        self.verbose
    }

    fn run(&self) -> anyhow::Result<()> {
        self.command.run()
    }
}

/// The main function.
fn main() -> Result<()> {
    Zinkc::start()
}
