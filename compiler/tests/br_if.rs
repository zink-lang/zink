//! br_if tests for the zink compiler.
#![cfg(test)]

use anyhow::Result;
use zinkc::Compiler;
use zint::{Bytes32, InstructionResult, EVM};

mod common;

#[test]
fn as_block_last() -> Result<()> {
    let wasm = common::load("br_if", "as_block_last")?;
    let bytecode = Compiler::default().compile(&wasm)?;

    tracing::trace!("bytecode: {:?}", hex::encode(&bytecode));

    let (ret, instr) = EVM::run(&bytecode, &42.to_bytes32());
    assert_eq!(instr, InstructionResult::Stop);
    assert_eq!(ret, []);

    Ok(())
}
