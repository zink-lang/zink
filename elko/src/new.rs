//! Command `New`
use crate::utils::Manifest;
use anyhow::{anyhow, Result};
use ccli::clap::{self, Parser};
use colored::*;
use std::{fs, path::PathBuf};

const NAME: &str = "${name}";
const ADDITION: &str = r#"
//! ${name}
#![no_std]

// For the panic handler.
#[cfg(not(test))]
extern crate zink;

/// Adds two numbers together.
#[no_mangle]
pub extern "C" fn addition(x: u64, y: u64) -> u64 {
    x + y
}
"#;
const README: &str = r#"
# ${name}

> An EVM contract written in Rust with [The Zink Project][zink].

## Getting Started

```
cargo install zinkup
elko build
ls target/zink/${name}.bin
```

[zink]: https://github.com/clearloop/zink
"#;

/// Create a new zink project.
#[derive(Debug, Parser)]
pub struct New {
    path: PathBuf,

    /// The name of the project.
    #[clap(short, long, value_name = "name")]
    name: Option<String>,
}

impl New {
    /// Get the project name.
    pub fn name(&self) -> Result<String> {
        let name = if let Some(name) = self.name.as_ref() {
            name.into()
        } else {
            self.path
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| anyhow!("Invalid project path"))?
                .into()
        };

        Ok(name)
    }

    /// Create a new zink project.
    pub fn run(&self) -> Result<()> {
        fs::create_dir_all(&self.path)?;

        // Create the cargo manifest.
        let mut manifest = Manifest::default();
        let name = self.name()?;
        manifest.name(&name);
        fs::write(
            self.path.join("Cargo.toml"),
            toml::to_string_pretty(&manifest)?,
        )?;

        // Create the src directory.
        let src = self.path.join("src");
        fs::create_dir_all(&src)?;
        fs::write(
            src.join("lib.rs"),
            ADDITION.trim_start().replace(NAME, &name),
        )?;

        // Create README
        fs::write(
            self.path.join("README.md"),
            README.trim_start().replace(NAME, &name),
        )?;

        println!("{} zink project `{}`", "Created".green().bold(), name);
        Ok(())
    }
}
