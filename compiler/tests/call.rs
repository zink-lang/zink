//! if-else tests for the zink compiler.
#![cfg(test)]

use anyhow::Result;
use zint::{Bytes32, InstructionResult, EVM};

mod common;

#[test]
fn dummy() -> Result<()> {
    let bytecode = common::load("call", "dummy")?;
    let info = EVM::run(&bytecode, &[]);

    assert_eq!(info.instr, InstructionResult::Return);
    assert_eq!(info.ret, []);

    Ok(())
}

#[test]
fn params() -> Result<()> {
    let bytecode = common::load("call", "params")?;

    let input = [1.to_bytes32(), 2.to_bytes32()].concat();
    let info = EVM::run(&bytecode, &input);

    assert_eq!(info.ret, 3.to_bytes32());
    Ok(())
}

#[test]
fn as_if() -> Result<()> {
    let bytecode = common::load("call", "as_if")?;
    let info = EVM::run(&bytecode, &0.to_bytes32());
    assert_eq!(info.ret, 0.to_bytes32());

    let info = EVM::run(&bytecode, &1.to_bytes32());
    assert_eq!(info.ret, 1.to_bytes32());

    let info = EVM::run(&bytecode, &2.to_bytes32());
    assert_eq!(info.ret, 41.to_bytes32());

    let info = EVM::run(&bytecode, &3.to_bytes32());
    assert_eq!(info.ret, 42.to_bytes32());
    Ok(())
}
