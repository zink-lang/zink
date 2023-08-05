pub use self::{bump::Bump, publish::Publish};
use anyhow::Result;
use clap::Parser;

mod bump;
mod publish;

/// Commands of this tool.
#[derive(Debug, Parser, Clone)]
pub enum Command {
    Bump(Bump),
    Publish(Publish),
}

/// Modern tool for bumping crate versions and
/// publishing them.
#[derive(Debug, Parser, Clone)]
pub struct Conta {
    #[clap(subcommand)]
    command: Command,
}

impl Conta {
    /// Process commands
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Command::Bump(bump) => bump.run()?,
            Command::Publish(_) => {}
        }

        Ok(())
    }
}
