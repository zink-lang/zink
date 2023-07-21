//! Testing fibonacci
#![cfg(test)]

use anyhow::Result;
use zinkc::Compiler;
use zint::EVM;

mod common;

#[ignore]
#[test]
fn recursion() -> Result<()> {
    let wasm = common::load("", "fibonacci")?;
    let bytecode = Compiler::default().compile(&wasm)?;

    // f(0) = 0;
    let (ret, _) = EVM::run(&bytecode, &[0; 32]);
    assert_eq!(ret, [0; 32]);

    Ok(())
}
