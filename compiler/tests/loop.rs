//! loop tests

use anyhow::Result;
use zinkc::Compiler;
use zint::{Bytes32, EVM};

mod common;

#[test]
fn singular() -> Result<()> {
    let wasm = common::load("loop", "singular")?;
    let bytecode = Compiler::default().compile(&wasm)?;

    let info = EVM::run(&bytecode, &[]);
    assert_eq!(info.ret, 7.to_bytes32());

    Ok(())
}

#[ignore]
#[test]
fn recursion() -> Result<()> {
    let wasm = common::load("loop", "fibonacci")?;
    let bytecode = Compiler::default().compile(&wasm)?;

    // f(0) = 0;
    let info = EVM::run(&bytecode, &[0; 32]);
    assert_eq!(info.ret, [0; 32]);

    Ok(())
}
