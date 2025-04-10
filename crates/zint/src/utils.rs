use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::path::PathBuf;

/// Represents the Foundry configuration (foundry.toml)
#[derive(Deserialize)]
pub struct FoundryConfig {
    /// The profile section of the `foundry.toml` configuration.
    pub profile: Profile,
}

/// Represents a profile in the Foundry configuration.
#[derive(Deserialize)]
pub struct Profile {
    /// The default profile settings.
    pub default: ProfileSettings,
}

/// Represents the settings for a Foundry profile.
#[derive(Deserialize)]
pub struct ProfileSettings {
    /// The output directory for compiled artifacts.
    pub out: Option<String>,
}

/// Find a file by walking up the directory tree
pub fn find_up(filename: &str) -> Result<PathBuf> {
    let mut path = std::env::current_dir()?;
    loop {
        let candidate = path.join(filename);
        if candidate.exists() {
            return Ok(candidate);
        }
        if !path.pop() {
            return Err(anyhow!(
                "Could not find {} in current or parent directories",
                filename
            ));
        }
    }
}
