//! Tests for instruction `select`.

use anyhow::Result;
use zint::EVM;

mod common;

#[ignore]
#[test]
fn params() -> Result<()> {
    let bytecode = common::load("log", "log0")?;

    // returns the bigger number.
    let _info = EVM::run(&bytecode, &[]);

    Ok(())
}
