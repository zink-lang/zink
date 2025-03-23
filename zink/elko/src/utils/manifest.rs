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
        // Use the ZINK_VERSION set by build.rs, fallback to 0.1.0 if not available
        let zink_version = option_env!("ZINK_VERSION").unwrap_or("0.1.0");
        Self {
            zink: Version::parse(zink_version).expect("Invalid zink version format"),
        }
    }
}

/// Dev-dependencies of the cargo project.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DevDependencies {
    pub zint: Version,
}

impl Default for DevDependencies {
    fn default() -> Self {
        let zint_version = option_env!("ZINK_VERSION").unwrap_or("0.1.0");
        Self {
            zint: Version::parse(zint_version).expect("Invalid zint version format"),
        }
    }
}

/// Empty workspace to indicate this crate is standalone.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Workspace {}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            package: Package::default(),
            lib: Lib::default(),
            dependencies: Dependencies::default(),
            dev_dependencies: Some(DevDependencies::default()),
            workspace: Workspace::default(),
        }
    }
}

/// Cargo Manifest for the zink project.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Manifest {
    /// Package section of cargo manifest.
    pub package: Package,
    /// Lib section of cargo manifest.
    pub lib: Lib,
    /// Dependencies of the cargo project.
    pub dependencies: Dependencies,
    /// Dev-dependencies of the cargo project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dev_dependencies: Option<DevDependencies>,
    /// Empty workspace to indicate this crate is standalone.
    #[serde(default)]
    pub workspace: Workspace,
}

impl Manifest {
    /// Set package name
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.package.name = name.to_string();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_zink_version() {
        let manifest = Manifest::default();
        let expected_version = option_env!("ZINK_VERSION")
            .unwrap_or("0.1.0")
            .parse::<Version>()
            .expect("Invalid version format");
        assert_eq!(
            manifest.dependencies.zink, expected_version,
            "Zink version should match the one set by build.rs or fallback to 0.1.0"
        );

        // Check that it’s not hardcoded to 0.1.0 unless fallback is triggered
        if option_env!("ZINK_VERSION").is_some() {
            assert_ne!(
                manifest.dependencies.zink,
                Version::new(0, 1, 0),
                "Zink version should reflect build.rs output, not the hardcoded fallback"
            );
        }
    }

    #[test]
    fn test_manifest_serialization() {
        let mut manifest = Manifest::default();
        manifest.name("testproj");
        let toml = toml::to_string_pretty(&manifest).expect("Failed to serialize manifest");
        assert!(toml.contains("name = \"testproj\""));
        assert!(toml.contains(&format!(
            "zink = \"{}\"",
            option_env!("ZINK_VERSION").unwrap_or("0.1.0")
        )));
    }
}
