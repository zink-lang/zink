pub use crate::{
    cmd::{bump::Bump, publish::Publish},
    Config,
};
use anyhow::Result;
use ccli::{
    clap::{self, Parser},
    App,
};
use std::{fs, path::PathBuf};

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
    /// The path of the cargo manifest, if not provided, the
    /// current directory is used.
    #[clap(short, long)]
    manifest: Option<PathBuf>,

    /// The path of `Conta.toml`
    #[clap(short, long)]
    config: Option<PathBuf>,

    /// The command to run.
    #[clap(subcommand)]
    command: Command,
}

impl Conta {
    /// Get the manifest path.
    pub fn manifest(&self) -> PathBuf {
        if let Some(p) = &self.manifest {
            p.into()
        } else {
            PathBuf::from("Cargo.toml")
        }
    }

    /// Parse the config from the input path.
    pub fn config(&self) -> Result<Config> {
        let path = if let Some(p) = &self.config {
            p.into()
        } else {
            PathBuf::from("Conta.toml")
        };

        toml::from_str(&fs::read_to_string(path)?).map_err(Into::into)
    }
}

impl App for Conta {
    fn verbose(&self) -> u8 {
        0
    }

    fn run(&self) -> Result<()> {
        let manifest = self.manifest();
        let config = self.config()?;

        match &self.command {
            Command::Bump(bump) => bump.run(&manifest, config),
            Command::Publish(publish) => publish.run(&manifest, config.packages),
        }
    }
}
