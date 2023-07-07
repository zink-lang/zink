//! Shared module for tests.

use anyhow::Result;
use std::{fs, path::PathBuf};
use tracing::trace;
use tracing_subscriber::EnvFilter;
use wat;

fn setup_logger() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .compact()
        .try_init()
        .ok();
}

/// Load wat as wasm binary from path.
pub fn load(instr: &str, name: &str) -> Result<Vec<u8>> {
    setup_logger();

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("wat/{instr}/{name}.wat"));
    trace!("Loading {path:?}");

    let wat = fs::read(path)?;
    Ok(wat::parse_bytes(&wat).map(Into::into)?)
}
