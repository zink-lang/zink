//! if-else tests for the zink compiler.
#![cfg(test)]

use anyhow::Result;
use zinkc::Compiler;
use zint::{Bytes32, EVM};

mod common;

#[test]
fn if_then() -> Result<()> {
    let wasm = common::load("if", "basic")?;
    let bytecode = Compiler::default().compile(&wasm)?;

    // Skip the condition.
    let info = EVM::run(&bytecode, &[0; 32]);
    assert_eq!(info.ret, [0; 32]);

    // Enter the if branch.
    let input = 1.to_bytes32();
    let info = EVM::run(&bytecode, &input);
    assert_eq!(info.ret, input);

    Ok(())
}

#[test]
fn singular() -> Result<()> {
    let wasm = common::load("if", "singular")?;
    let bytecode = Compiler::default().compile(&wasm)?;

    // test if
    let info = EVM::run(&bytecode, &0.to_bytes32());
    assert_eq!(info.ret, 7.to_bytes32());

    // test else
    let info = EVM::run(&bytecode, &1.to_bytes32());
    assert_eq!(info.ret, 8.to_bytes32());

    Ok(())
}
