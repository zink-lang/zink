//! Addition tests for the zink compiler.
#![cfg(test)]

use anyhow::Result;
use zint::{Bytes32, EVM};

mod common;

#[test]
fn params() -> Result<()> {
    let bytecode = common::load("i32add", "params")?;

    // add(1, 2)
    let input = [1.to_bytes32(), 2.to_bytes32()].concat();
    let info = EVM::run(&bytecode, &input);

    assert_eq!(info.ret, [3.to_bytes32()].concat());
    Ok(())
}

#[test]
fn locals() -> Result<()> {
    let bytecode = common::load("i32add", "locals")?;
    let info = EVM::run(&bytecode, &[]);

    assert_eq!(info.ret, [30.to_bytes32()].concat());
    Ok(())
}

#[test]
fn tee() -> Result<()> {
    let bytecode = common::load("i32add", "tee")?;
    let info = EVM::run(&bytecode, &[]);

    assert_eq!(info.ret, [30.to_bytes32()].concat());
    Ok(())
}
