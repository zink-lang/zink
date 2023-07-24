//! if-else tests for the zink compiler.
#![cfg(test)]

use anyhow::Result;
use zinkc::Compiler;
use zint::{Bytes32, InstructionResult, EVM};

mod common;

#[test]
fn params() -> Result<()> {
    let wasm = common::load("call", "params")?;
    let bytecode = Compiler::default().compile(&wasm)?;

    let input = [1.to_bytes32(), 2.to_bytes32()].concat();
    let info = EVM::run(&bytecode, &input);
    assert_eq!(info.ret, 3.to_bytes32());
    Ok(())
}

#[test]
fn dummy() -> Result<()> {
    let wasm = common::load("call", "dummy")?;
    let bytecode = Compiler::default().compile(&wasm)?;

    tracing::trace!("bytecode: {:?}", hex::encode(&bytecode));

    let info = EVM::run(&bytecode, &[]);
    assert_eq!(info.instr, InstructionResult::Return);
    assert_eq!(info.ret, []);
    Ok(())
}
