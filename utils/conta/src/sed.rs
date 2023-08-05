//! Manifest reader

use anyhow::{anyhow, Result};
use semver::{Version, VersionReq};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

const WORKSPACE_PACKAGE: &str = "[workspace.package]";
const WORKSPACE_DEPENDENCIES: &str = "[workspace.dependencies]";
const PATT_VERSION: &str = "version = \"";

/// Position of version field
///
/// This is used to determine the position of the version
/// field in the manifest file.
///
/// # Example
///
/// ```toml
/// [workspace.package]
/// version = "0.1.0"
/// ```
///
/// The range of Pos will be the first and the second quote.
#[derive(Debug, Clone)]
pub struct Pos {
    /// Start position
    pub start: usize,
    /// End position
    pub end: usize,
}

/// Manifest stream editor.
pub struct Sed {
    pub buf: Vec<u8>,
    manifest: PathBuf,
    root: Pos,
    map: BTreeMap<String, Pos>,
}

impl Sed {
    /// Create a new manifest stream editor.
    pub fn new(p: impl AsRef<Path>, packages: &[String]) -> Result<Self> {
        let manifest = p.as_ref().into();
        let buf = fs::read(&manifest)?;
        let content = String::from_utf8_lossy(&buf);

        let find_version = |start: usize, patt: &str| -> Result<Pos> {
            let start = content[start..]
                .find(patt)
                .ok_or(anyhow!("pattern {patt} not found"))?
                + start
                + patt.len();

            let start = content[start..]
                .find("version = \"")
                .ok_or(anyhow!("version not found"))?
                + start
                + PATT_VERSION.len();

            let end = content[start..]
                .find("\"")
                .ok_or(anyhow!("the end of version field is invalid"))?
                + start;

            Ok(Pos { start, end })
        };

        // Get the version position of the root package.
        let root = find_version(0, WORKSPACE_PACKAGE)?;

        // Get the version position of each package.
        let map = {
            let start = content
                .find(WORKSPACE_DEPENDENCIES)
                .ok_or(anyhow!("workspace dependencies not found"))?
                + WORKSPACE_DEPENDENCIES.len();

            packages
                .iter()
                .map(|package| {
                    find_version(start, &format!("{package} ")).map(|pos| (package.clone(), pos))
                })
                .collect::<Result<_>>()?
        };

        Ok(Self {
            buf,
            manifest,
            root,
            map,
        })
    }

    /// Set the version from pos.
    pub fn set_version(&mut self, version: &str, pos: Pos) -> Result<()> {
        let Pos { start, end } = pos;

        let buf = self.buf.clone();
        let (before, after) = buf.split_at(start);
        let (_, after) = after.split_at(end - start);

        self.buf = [before, version.as_bytes(), after].concat();

        Ok(())
    }

    /// Set the version of the root package.
    pub fn set_dep_versions(&mut self, version: &VersionReq) -> Result<()> {
        for (_, pos) in self.map.clone().into_iter() {
            self.set_version(&version.to_string(), pos)?;
        }

        Ok(())
    }

    /// Set the version of the root package.
    pub fn set_workspace_version(&mut self, version: &Version) -> Result<()> {
        self.set_version(&version.to_string(), self.root.clone())
    }

    /// Flush the changes to the manifest file.
    pub fn flush(self) -> Result<()> {
        fs::write(self.manifest, self.buf)?;
        Ok(())
    }
}
