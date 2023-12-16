//! if-else tests for the zink compiler.
#![cfg(test)]

use anyhow::Result;
use filetests::Test;
use zint::{Bytes32, Contract, InstructionResult};

#[test]
fn dummy() -> Result<()> {
    let mut contract = Contract::new(Test::CALL_DUMMY)
        .without_dispatcher()
        .compile()?;
    let info = contract.execute::<()>([])?;

    assert_eq!(info.instr, InstructionResult::Return);
    assert!(info.ret.is_empty());
    Ok(())
}

#[test]
fn params() -> Result<()> {
    let mut contract = Contract::new(Test::CALL_PARAMS)
        .without_dispatcher()
        .compile()?;
    let info = contract.execute([1, 2])?;

    assert_eq!(info.ret, 3.to_bytes32());
    Ok(())
}

#[test]
fn as_if() -> Result<()> {
    let mut contract = Contract::new(Test::CALL_AS_IF)
        .without_dispatcher()
        .compile()?;

    let info = contract.execute([0])?;
    assert_eq!(info.ret, 0.to_bytes32());

    let info = contract.execute([1])?;
    assert_eq!(info.ret, 1.to_bytes32());

    let info = contract.execute([2])?;
    assert_eq!(info.ret, 41.to_bytes32());

    let info = contract.execute([3])?;
    assert_eq!(info.ret, 42.to_bytes32());
    Ok(())
}
