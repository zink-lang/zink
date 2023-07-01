//! Zink CLI utils
use crate::App;
use clap::Parser;
use color_eyre::{eyre::eyre, Result};
use env_logger::{Builder, Env};

/// Setup app as binary.
pub fn app<T>() -> Result<()>
where
    T: Parser + App,
{
    color_eyre::install()?;

    let app = T::parse();
    let env = Env::default().default_filter_or(match app.verbose() {
        0 => "info",
        1 => "warn",
        2 => "debug",
        _ => "trace",
    });

    Builder::from_env(env).init();

    app.run().map_err(|e| eyre!("Failed to run app, {e}"))?;
    Ok(())
}
