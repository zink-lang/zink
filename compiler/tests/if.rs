//! if-else tests for the zink compiler.
#![cfg(test)]

use anyhow::Result;
use zinkc::Compiler;
use zint::EVM;

mod common;

#[test]
fn basic() -> Result<()> {
    let wasm = common::load("if", "basic")?;
    let bytecode = Compiler::default().compile(&wasm)?;

    tracing::trace!("{:x?}", bytecode);

    // Skip the condition.
    let (ret, _) = EVM::run(&bytecode, &[0; 32]);
    assert_eq!(ret, [0; 32]);

    // Enter the if branch.
    let mut input = vec![0; 31];
    input.push(1);
    let (ret, _) = EVM::run(&bytecode, &input);
    assert_eq!(ret, input);

    Ok(())
}
