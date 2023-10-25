use anyhow::{anyhow, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};
use zinkc::Compiler;

/// Read the contents of a directory, returning
/// all wat files.
fn find_wat(dir: impl AsRef<Path>, files: &mut Vec<PathBuf>) -> Result<()> {
    let entry = fs::read_dir(dir)?;
    for entry in entry {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            find_wat(path, files)?;
        } else if path.extension().unwrap_or_default() == "wat" {
            files.push(path);
        }
    }

    Ok(())
}

#[test]
fn filetests() -> Result<()> {
    zint::setup_logger();

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("wat"));
    let mut files = Vec::new();
    find_wat(&path, &mut files)?;

    let excludes = ["as_if_else.wat"];

    for wat in files.iter().filter(|f| {
        !excludes.contains(
            &f.file_name()
                .map(|n| n.to_str())
                .flatten()
                .expect("file name not found"),
        )
    }) {
        let wat_bytes = fs::read(&wat)?;
        let wasm = wat::parse_bytes(&wat_bytes)?;

        Compiler::default()
            .compile(&wasm)
            .map_err(|e| anyhow!("Failed to compile {:?}, {e}", wat))?;
    }

    Ok(())
}
