//! storage tests
#![cfg(test)]

use anyhow::Result;
use filetests::Test;
use zint::{keccak256, Bytes32, Contract, EVM, U256};

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

#[test]
fn mapping() -> Result<()> {
    use opcodes::ShangHai;
    use zint::Bytes32;

    zint::setup_logger();

    let hashing: Vec<u8> = vec![
        // storage value
        ShangHai::PUSH1,
        ShangHai::Data(0x42),
        // Load storage slot
        //
        // write index to memory
        ShangHai::PUSH0,
        ShangHai::PUSH0,
        ShangHai::MSTORE8,
        // write key to memory
        ShangHai::PUSH0,
        ShangHai::PUSH1,
        ShangHai::Data(0x01),
        ShangHai::MSTORE, 
        // hash key
        ShangHai::PUSH1,
        ShangHai::Data(0x20),
        ShangHai::PUSH0,
        ShangHai::KECCAK256,
        // write storage
        ShangHai::SSTORE,
        // Load storage slot
        //
        // write index to memory
        ShangHai::PUSH0,
        ShangHai::PUSH0,
        ShangHai::MSTORE8,
        // write key to memory
        ShangHai::PUSH0,
        ShangHai::PUSH1,
        ShangHai::Data(0x01),
        ShangHai::MSTORE,
        // hash key
        ShangHai::PUSH1,
        ShangHai::Data(0x20),
        ShangHai::PUSH0,
        ShangHai::KECCAK256,
        // load storage to stack
        ShangHai::SLOAD,
        // write storage to memory
        ShangHai::PUSH0,
        ShangHai::MSTORE,
        // return
        ShangHai::PUSH1,
        ShangHai::Data(0x20),
        ShangHai::PUSH0,
        ShangHai::RETURN,
    ]
    .into_iter()
    .map(Into::into)
    .collect();

    let info = EVM::interp(&hashing, &[])?;
    tracing::debug!("bytecode: {}", hex::encode(&hashing));

    let key = keccak256(&[0; 0x20]);
    assert_eq!(
        info.storage.get(&U256::from_be_bytes(key)),
        Some(&U256::from_be_bytes(0x42.to_bytes32())),
        "{info:#?}"
    );
    assert_eq!(0x42.to_bytes32().to_vec(), info.ret);
    Ok(())
}
