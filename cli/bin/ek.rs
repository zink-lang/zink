//! Zink Compiler

use clap::{Parser, Subcommand};
use color_eyre::Result;
use zinkup::{App, Build};

/// ek commands
#[derive(Debug, Subcommand)]
enum Command {
    Build(Build),
}

/// Zink's package manager
#[derive(Debug, Parser)]
#[command(name = "ek", version)]
pub struct Ek {
    #[command(subcommand)]
    command: Command,
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

impl App for Ek {
    fn verbose(&self) -> u8 {
        self.verbose
    }

    fn run(&self) -> anyhow::Result<()> {
        match &self.command {
            Command::Build(build) => build.run(),
        }
    }
}

fn main() -> Result<()> {
    Ek::start()
}
