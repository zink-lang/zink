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

#[test]
fn recursion() -> Result<()> {
    let bytecode = common::load("loop", "fibonacci")?;

    // f(0) = 0;
    let info = EVM::run(&bytecode, &1.to_bytes32());
    assert_eq!(1.to_bytes32().to_vec(), info.ret);

    let info = EVM::run(&bytecode, &3.to_bytes32());
    // assert_eq!(3.to_bytes32().to_vec(), info.ret);
    tracing::trace!("{info:?}");

    Ok(())
}
