//! if-else tests for the zink compiler.
#![cfg(test)]

use anyhow::Result;
use zinkc::Compiler;
use zint::EVM;

mod common;

#[test]
fn params() -> Result<()> {
    let wasm = common::load("call", "params")?;
    let bytecode = Compiler::default().compile(&wasm)?;

    let input = [vec![0; 31], vec![1; 1], vec![0; 31], vec![2; 1]].concat();
    let (ret, _) = EVM::run(&bytecode, &input);
    assert_eq!(ret, [vec![0; 31], vec![3; 1]].concat());
    Ok(())
}
