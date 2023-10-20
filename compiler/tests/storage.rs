//! storage tests
#![cfg(test)]

use anyhow::Result;
use zint::{Bytes32, InstructionResult, EVM, U256};

mod common;

#[test]
fn store() -> Result<()> {
    let bytecode = common::load("storage", "store")?;
    let key = 0;
    let value = 42;
    let info = EVM::run(&bytecode, &value.to_bytes32());

    assert_eq!(info.instr, InstructionResult::Return);
    assert_eq!(info.ret, []);
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
    let _bytecode = common::load_with_dispatcher("storage", "dispatcher")?;

    // TODO: testing set (#122)
    // {
    //     let key = 0;
    //     let value = 42;
    //     let mut selector = zabi::selector(b"set(i32)").to_vec();
    //     selector = [selector, 42.to_bytes32().to_vec()].concat();
    //     let info = EVM::run(&bytecode, &selector);
    //     assert_eq!(info.instr, InstructionResult::Return);
    //     assert_eq!(info.ret, []);
    //     assert_eq!(info.storage.get(&U256::from(key)), Some(&U256::from(value)));
    // }

    // let info = EVM::run(&bytecode, &42.to_bytes32());

    // assert_eq!(info.instr, InstructionResult::Return);
    // assert_eq!(info.ret, 42.to_bytes32());

    Ok(())
}
