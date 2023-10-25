//! storage tests
#![cfg(test)]

use anyhow::Result;
use zint::{Bytes32, Contract, InstructionResult, EVM, U256};

mod common;

#[test]
fn store() -> Result<()> {
    let bytecode = common::load("storage", "store")?;
    let key = 0;
    let value = 42;
    let info = EVM::run(&bytecode, &value.to_bytes32());

    assert!(info.ret.is_empty());
    assert_eq!(info.instr, InstructionResult::Return);
    assert_eq!(info.storage.get(&U256::from(key)), Some(&U256::from(value)));

    Ok(())
}

#[test]
fn load() -> Result<()> {
    let bytecode = common::load("storage", "load")?;
    let value = 42.to_bytes32();
    let info = EVM::run(&bytecode, &value);

    assert_eq!(info.instr, InstructionResult::Return);
    assert_eq!(info.ret, value);

    Ok(())
}

#[test]
fn basic() -> Result<()> {
    let bytecode = common::load("storage", "basic")?;
    let info = EVM::run(&bytecode, &42.to_bytes32());

    assert_eq!(info.instr, InstructionResult::Return);
    assert_eq!(info.ret, 42.to_bytes32());

    Ok(())
}

#[test]
fn dispatcher() -> Result<()> {
    let mut contract = Contract::new(common::load_wasm("storage", "dispatcher")?).compile()?;

    {
        let key = 0;
        let value: i32 = 42;
        let info = contract.execute(&[b"set(i32)".to_vec(), value.to_bytes32().to_vec()])?;
        assert!(info.ret.is_empty());
        assert_eq!(info.instr, InstructionResult::Return);
        assert_eq!(info.storage.get(&U256::from(key)), Some(&U256::from(value)));
    }

    {
        let info = contract.execute(&["get()"])?;
        assert_eq!(info.instr, InstructionResult::Return);
        assert_eq!(info.ret, 0.to_bytes32());
    }

    {
        let key = 0;
        let value = 42;
        let info =
            contract.execute(&[b"set_and_get(i32)".to_vec(), value.to_bytes32().to_vec()])?;
        assert_eq!(info.instr, InstructionResult::Return);
        assert_eq!(info.ret, value.to_bytes32());
        assert_eq!(info.storage.get(&U256::from(key)), Some(&U256::from(value)));
    }

    Ok(())
}
