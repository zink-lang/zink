//! if-else tests for the zink compiler.
#![cfg(test)]

use anyhow::Result;
use zinkc::Compiler;

mod common;

#[test]
fn params() -> Result<()> {
    let wasm = common::load("call", "params")?;
    let _bytecode = Compiler::compile(&wasm)?;

    // // Skip the condition.
    // let (ret, _) = EVM::run(&bytecode, &[0; 32]);
    // assert_eq!(ret, [0; 32]);
    //
    // // Enter the if branch.
    // let mut input = vec![0; 31];
    // input.push(1);
    // let (ret, _) = EVM::run(&bytecode, &input);
    // assert_eq!(ret, input);

    Ok(())
}
