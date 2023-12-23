//! The package manager of zink.
#![deny(missing_docs)]

use ccli::{clap::Subcommand, App, Parser, Result};
use elko::{Build, Compile, New};

/// Elko commands
#[derive(Debug, Subcommand)]
enum Command {
    New(New),
    Build(Build),
    Compile(Compile),
}

/// The package manager of zink.
#[derive(Debug, Parser)]
#[command(name = "elko", version)]
pub struct Elko {
    #[command(subcommand)]
    command: Command,
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

impl App for Elko {
    fn verbose(&self) -> u8 {
        self.verbose
    }

    fn run(&self) -> anyhow::Result<()> {
        match &self.command {
            Command::Build(build) => build.run(),
            Command::New(new) => new.run(),
            Command::Compile(compile) => compile.run(),
        }
    }
}

/// The main function.
fn main() -> Result<()> {
    Elko::start()
}
