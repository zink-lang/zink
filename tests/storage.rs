//! storage tests
#![cfg(test)]

use anyhow::Result;
use zinkc_filetests::Test;
use zint::{Bytes32, Contract, InstructionResult, U256};

#[test]
fn store() -> Result<()> {
    let mut contract = Contract::new(Test::STORAGE_STORE)
        .without_dispatcher()
        .compile()?;

    let key = 0;
    let value = 42;
    let info = contract.execute([value])?;
    assert!(info.ret.is_empty());
    assert_eq!(info.instr, InstructionResult::Return);
    assert_eq!(info.storage.get(&U256::from(key)), Some(&U256::from(value)));

    Ok(())
}

#[test]
fn load() -> Result<()> {
    let mut contract = Contract::new(Test::STORAGE_LOAD)
        .without_dispatcher()
        .compile()?;

    let value = 42;
    let info = contract.execute([value])?;
    assert_eq!(info.instr, InstructionResult::Return);
    assert_eq!(info.ret, value.to_bytes32());

    Ok(())
}

#[test]
fn basic() -> Result<()> {
    let mut contract = Contract::new(Test::STORAGE_BASIC)
        .without_dispatcher()
        .compile()?;

    let value = 42;
    let info = contract.execute([value])?;
    assert_eq!(info.instr, InstructionResult::Return);
    assert_eq!(info.ret, 42.to_bytes32());

    Ok(())
}
