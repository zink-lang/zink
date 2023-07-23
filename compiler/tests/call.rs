//! if-else tests for the zink compiler.
#![cfg(test)]

use anyhow::Result;
use zinkc::Compiler;
use zint::{InstructionResult, EVM};

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

#[test]
fn dummy() -> Result<()> {
    let wasm = common::load("call", "dummy")?;
    let bytecode = Compiler::default().compile(&wasm)?;

    tracing::trace!("bytecode: {:?}", hex::encode(&bytecode));

    let (ret, instr) = EVM::run(&bytecode, &[]);
    assert_eq!(instr, InstructionResult::Stop);
    assert_eq!(ret, &[]);
    Ok(())
}
