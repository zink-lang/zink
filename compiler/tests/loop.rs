//! loop tests

use anyhow::Result;
use zint::{Bytes32, EVM};

mod common;

#[test]
fn singular() -> Result<()> {
    let bytecode = common::load("loop", "singular")?;

    let info = EVM::run(&bytecode, &[]);
    assert_eq!(info.ret, 7.to_bytes32());

    Ok(())
}

#[ignore]
#[test]
fn recursion() -> Result<()> {
    let bytecode = common::load("loop", "fibonacci")?;

    // f(0) = 0;
    let info = EVM::run(&bytecode, &[0; 32]);
    assert_eq!(info.ret, [0; 32]);

    Ok(())
}
