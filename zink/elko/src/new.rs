//! Command `New`
use crate::examples;
use crate::utils::Manifest;
use anyhow::{anyhow, Result};
use ccli::clap::{self, Parser};
use colored::*;
use std::{fs, path::PathBuf};

/// Create a new zink project.
#[derive(Debug, Parser)]
pub struct New {
    path: PathBuf,
    /// The name of the project.
    #[clap(short, long, value_name = "name")]
    name: Option<String>,
    /// Initialize the project with an example (e.g., "addition", "erc20").
    #[clap(long, value_name = "example")]
    example: Option<String>,
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
        let name = self.name()?;

        // Select the example based on the --example flag
        let example = match self.example.as_deref() {
            Some("erc20") => examples::ERC20,
            Some("addition") | None => examples::ADDITION,
            Some(unknown) => return Err(anyhow!("Unknown example: {}", unknown)),
        };

        // Create the cargo manifest
        let mut manifest = Manifest::default();
        manifest.name(&name);
        fs::write(
            self.path.join("Cargo.toml"),
            toml::to_string_pretty(&manifest)?,
        )?;

        // Create the src directory and lib.rs
        let src = self.path.join("src");
        fs::create_dir_all(&src)?;
        fs::write(
            src.join("lib.rs"),
            example.lib_rs.trim_start().replace("${name}", &name),
        )?;

        // Create README
        fs::write(
            self.path.join("README.md"),
            example.readme.trim_start().replace("${name}", &name),
        )?;

        println!(
            "{} zink project `{}` with example `{}`",
            "Created".green().bold(),
            name,
            self.example.as_deref().unwrap_or("addition")
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_new_addition() -> Result<()> {
        let tmp = tempfile::tempdir()?;
        let new = New {
            path: tmp.path().join("myproj"),
            name: Some("myproj".to_string()),
            example: None,
        };
        new.run()?;
        assert!(Path::new(&tmp.path().join("myproj/src/lib.rs")).exists());
        let lib_rs = fs::read_to_string(tmp.path().join("myproj/src/lib.rs"))?;
        assert!(lib_rs.contains("addition(x: u64, y: u64)"));
        Ok(())
    }

    #[test]
    fn test_new_erc20() -> Result<()> {
        let tmp = tempfile::tempdir()?;
        let new = New {
            path: tmp.path().join("mytoken"),
            name: Some("mytoken".to_string()),
            example: Some("erc20".to_string()),
        };
        new.run()?;
        assert!(Path::new(&tmp.path().join("mytoken/src/lib.rs")).exists());
        let lib_rs = fs::read_to_string(tmp.path().join("mytoken/src/lib.rs"))?;
        assert!(lib_rs.contains("transfer(to: Address, value: U256)"));
        assert!(lib_rs.contains("approve(spender: Address, value: U256)"));
        Ok(())
    }

    #[test]
    fn test_new_invalid_example() {
        let tmp = tempfile::tempdir().unwrap();
        let new = New {
            path: tmp.path().join("myproj"),
            name: Some("myproj".to_string()),
            example: Some("invalid".to_string()),
        };
        assert!(new.run().is_err());
    }
}
