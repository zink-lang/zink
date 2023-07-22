//! loop tests

use anyhow::Result;
use zinkc::Compiler;
use zint::{Bytes32, EVM};

mod common;

#[test]
fn singular() -> Result<()> {
    let wasm = common::load("loop", "singular")?;
    let bytecode = Compiler::default().compile(&wasm)?;

    let (ret, _) = EVM::run(&bytecode, &[]);
    assert_eq!(ret, 7.to_bytes32());

    Ok(())
}
