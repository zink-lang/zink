//! loop tests

use anyhow::Result;
use zint::{Bytes32, InstructionResult, EVM};

mod common;

#[test]
fn singular() -> Result<()> {
    let bytecode = common::load("loop", "singular")?;

    let info = EVM::run(&bytecode, &[]);
    assert_eq!(info.ret, 7.to_bytes32());

    Ok(())
}

#[test]
fn as_br_if() -> Result<()> {
    let bytecode = common::load("loop", "as_br_if")?;

    let info = EVM::run(&bytecode, 0.to_bytes32().as_ref());
    assert_eq!(info.instr, InstructionResult::OutOfGas);

    let info = EVM::run(&bytecode, 1.to_bytes32().as_ref());
    assert_eq!(info.instr, InstructionResult::Return);
    assert_eq!(info.ret, 7.to_bytes32());

    Ok(())
}
