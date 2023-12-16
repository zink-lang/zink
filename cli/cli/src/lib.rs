//! Common command line interface.

pub use clap::{self, Parser};
pub use color_eyre::{eyre::eyre, Result};
use tracing_subscriber::filter::EnvFilter;

/// Shared application interface.
pub trait App: Parser {
    /// Verbose logging level.
    fn verbose(&self) -> u8;

    /// Run application.
    fn run(&self) -> anyhow::Result<()>;

    /// Start application.
    fn start() -> Result<()> {
        color_eyre::install()?;

        let app = Self::parse();
        let name = Self::command().get_name().to_string();
        let env =
            EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(match app.verbose() {
                0 => format!("{name}=info"),
                1 => format!("{name}=debug"),
                2 => "debug".into(),
                _ => "trace".into(),
            }));

        tracing_subscriber::fmt().with_env_filter(env).init();
        app.run().map_err(|e| eyre!("Failed to run app, {e}"))?;
        Ok(())
    }
}
