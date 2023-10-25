//! br_if tests for the zink compiler.
#![cfg(test)]

use anyhow::Result;
use zint::{Bytes32, InstructionResult, EVM};

mod common;

#[test]
fn as_block_last() -> Result<()> {
    let bytecode = common::load("br_if", "as_block_last")?;

    let info = EVM::run(&bytecode, &0.to_bytes32());
    assert_eq!(info.instr, InstructionResult::Return);
    assert!(info.ret.is_empty());

    let info = EVM::run(&bytecode, &42.to_bytes32());
    assert_eq!(info.instr, InstructionResult::OutOfGas);
    assert!(info.ret.is_empty());

    Ok(())
}
