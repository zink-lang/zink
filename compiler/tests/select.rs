//! Tests for instruction `select`.

use anyhow::Result;
use zint::{Bytes32, EVM};

mod common;

#[ignore]
#[test]
fn params() -> Result<()> {
    let bytecode = common::load("select", "params")?;

    // returns the bigger number.
    let input = [1.to_bytes32(), 2.to_bytes32()].concat();
    let info = EVM::run(&bytecode, &input);

    assert_eq!(info.ret, [2.to_bytes32()].concat());
    Ok(())
}
