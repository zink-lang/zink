//! Shared module for tests.

use anyhow::Result;
use std::{fs, path::PathBuf};
use tracing::trace;
use tracing_subscriber::EnvFilter;
use wat;
use zinkc::Compiler;

mod macros;

fn setup_logger() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .compact()
        .try_init()
        .ok();
}

/// Setup test environment.
fn compile(compiler: Compiler, instr: &str, name: &str) -> Result<Vec<u8>> {
    setup_logger();

    let wasm = load_wasm(instr, name)?;
    let bytecode = compiler.compile(&wasm)?.to_vec();
    tracing::trace!("bytecode: {:?}", hex::encode(&bytecode));
    Ok(bytecode)
}

/// Load wat to wasm
pub fn load_wasm(instr: &str, name: &str) -> Result<Vec<u8>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("wat/{instr}/{name}.wat"));
    trace!("Loading {path:?}");

    let wat = fs::read(path)?;
    wat::parse_bytes(&wat).map_err(Into::into).map(Into::into)
}

/// Load wat as wasm binary from path.
pub fn load(instr: &str, name: &str) -> Result<Vec<u8>> {
    compile(Compiler::default(), instr, name)
}

#[allow(unused)]
/// Load wat as wasm binary from path with dispatcher enabled.
pub fn load_with_dispatcher(instr: &str, name: &str) -> Result<Vec<u8>> {
    let mut compiler = Compiler::default().dispatcher(true);
    compile(compiler, instr, name)
}
