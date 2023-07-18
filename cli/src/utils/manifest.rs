//! Cargo Manifest for the zink project.
use semver::Version;
use serde::{Deserialize, Serialize};

/// Cargo package.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    /// Package name.
    pub name: String,
    /// Package version.
    pub version: Version,
    /// Package authors.
    pub authors: Vec<String>,
    /// Rust edition.
    pub edition: String,
}

impl Default for Package {
    fn default() -> Self {
        Self {
            name: "addition".to_string(),
            version: Version::new(0, 1, 0),
            authors: vec![],
            edition: "2021".to_string(),
        }
    }
}

/// Lib section of cargo manifest.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lib {
    /// The crate type of cargo project.
    #[serde(rename = "crate-type")]
    pub crate_type: Vec<String>,
}

impl Default for Lib {
    fn default() -> Self {
        Self {
            crate_type: vec!["cdylib".to_string()],
        }
    }
}

/// Dependencies of the cargo project.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dependencies {
    /// Zink dependency.
    pub zink: Version,
}

impl Default for Dependencies {
    fn default() -> Self {
        Self {
            // TODO: Get the version of zink from with build.rs.
            zink: Version::new(0, 1, 0),
        }
    }
}

/// Cargo Manifest for the zink project.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Manifest {
    /// Package section of cargo manifest.
    pub package: Package,
    /// Lib section of cargo manifest.
    pub lib: Lib,
    /// Dependencies of the cargo project.
    pub dependencies: Dependencies,
}

impl Manifest {
    /// Set package name
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.package.name = name.to_string();
        self
    }
}
