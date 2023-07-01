//! Zink CLI
use anyhow::Error;
use std::result::Result;

pub mod utils;

/// Shared application interface.
pub trait App {
    /// Verbose logging level.
    fn verbose(&self) -> u8;

    /// Run application.
    fn run(&self) -> Result<(), Error>;
}
