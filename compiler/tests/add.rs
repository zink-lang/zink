//! Addition tests for the zink compiler.
#![cfg(test)]

use anyhow::Result;
use zinkc::Compiler;
use zint::{Bytes32, EVM};

mod common;

#[test]
fn i32_add() -> Result<()> {
    let wasm = common::load("i32add", "params")?;
    let bytecode = Compiler::default().compile(&wasm)?;

    let input = [1.to_bytes32(), 2.to_bytes32()].concat();
    let (ret, _) = EVM::run(&bytecode, &input);

    assert_eq!(ret, [3.to_bytes32()].concat());
    Ok(())
}
