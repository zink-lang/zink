//! storage tests
#![cfg(test)]

use anyhow::Result;
use filetests::Test;
use zint::{Bytes32, Contract, U256};

#[test]
fn store() -> Result<()> {
    let mut contract = Contract::from(Test::STORAGE_STORE).pure().compile()?;

    let key = 0u64;
    let value = 42u64;
    let info = contract.execute([value])?;
    assert!(info.ret.is_empty());
    assert_eq!(info.storage.get(&U256::from(key)), Some(&U256::from(value)));

    Ok(())
}

#[test]
fn load() -> Result<()> {
    let mut contract = Contract::from(Test::STORAGE_LOAD).pure().compile()?;

    let value = 42;
    let info = contract.execute([value])?;
    assert_eq!(info.ret, value.to_bytes32());

    Ok(())
}

#[test]
fn basic() -> Result<()> {
    let mut contract = Contract::from(Test::STORAGE_BASIC).pure().compile()?;

    let value = 42;
    let info = contract.execute([value])?;
    assert_eq!(info.ret, 42.to_bytes32());

    Ok(())
}
