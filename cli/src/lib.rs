//! Zink command line tool
#![deny(missing_docs)]

pub use crate::build::Build;
use anyhow::Error;
use clap::Parser;
use color_eyre::{eyre::eyre, Result};
use tracing_subscriber::EnvFilter;

mod build;
mod utils;

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
        let env =
            EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(match app.verbose() {
                0 => format!("{name}=info"),
                1 => format!("{name}=debug"),
                2 => format!("{name}=trace"),
                _ => "trace".into(),
            }));

        tracing_subscriber::fmt().with_env_filter(env).init();
        app.run().map_err(|e| eyre!("Failed to run app, {e}"))?;
        Ok(())
    }
}
