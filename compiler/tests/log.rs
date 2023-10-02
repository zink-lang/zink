//! Tests for instruction `select`.

use anyhow::Result;
use zint::{Bytes32, EVM};

mod common;

#[test]
fn log0() -> Result<()> {
    let bytecode = common::load("log", "log0")?;

    // returns the bigger number.
    let info = EVM::run(&bytecode, &[]);
    assert_eq!(info.logs[0].data.to_vec(), b"Ping".to_vec().to_bytes32());
    Ok(())
}

#[test]
fn log1() -> Result<()> {
    let bytecode = common::load("log", "log1")?;

    // returns the bigger number.
    let info = EVM::run(&bytecode, &[]);
    assert_eq!(info.logs[0].data.to_vec(), b"Ping".to_vec().to_bytes32());
    assert_eq!(
        info.logs[0].topics[0].to_vec(),
        b"pong".to_vec().to_bytes32()
    );
    Ok(())
}
