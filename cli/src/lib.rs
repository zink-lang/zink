//! Zink CLI
pub use crate::zinkc::Zinkc;
use anyhow::Error;
use clap::Parser;
use color_eyre::{eyre::eyre, Result};
use env_logger::{Builder, Env};

mod zinkc;

/// Shared application interface.
pub trait App: Parser {
    /// Verbose logging level.
    fn verbose(&self) -> u8;

    /// Run application.
    fn run(&self) -> std::result::Result<(), Error>;

    /// Start application.
    fn start() -> Result<()> {
        color_eyre::install()?;

        let app = Self::parse();
        let name = Self::command().get_name().to_string();
        let env = Env::default().default_filter_or(match app.verbose() {
            0 => format!("{name}=info"),
            1 => format!("{name}=debug"),
            2 => format!("{name}=trace"),
            _ => "trace".into(),
        });

        Builder::from_env(env).init();

        app.run().map_err(|e| eyre!("Failed to run app, {e}"))?;
        Ok(())
    }
}
