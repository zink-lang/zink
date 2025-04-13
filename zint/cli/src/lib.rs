use anyhow::Result;
use clap::{Parser, Subcommand};

pub mod cmd;

#[derive(Parser)]
#[command(name = "cargo-zint")]
pub struct Cli {
    #[command(subcommand)]
    pub command: ZintCommand,
}

#[derive(Subcommand)]
pub enum ZintCommand {
    /// Zink testing commands
    Zint {
        #[command(subcommand)]
        subcommand: Commands,
    },
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new ztests crate in the Foundry project
    New,
    /// Run the tests in the ztests crate
    Run,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        ZintCommand::Zint { subcommand } => match subcommand {
            Commands::New => cmd::create_ztests_crate()?,
            Commands::Run => cmd::run_ztests()?,
        },
    }

    Ok(())
}
