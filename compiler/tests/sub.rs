//! Addition tests for the zink compiler.
#![cfg(test)]

use anyhow::Result;
use zint::{Bytes32, EVM};

mod common;

fn params(module: &str) -> Result<()> {
    let bytecode = common::load(module, "params")?;

    // add(1, 2)
    let input = [2.to_bytes32(), 1.to_bytes32()].concat();
    let info = EVM::run(&bytecode, &input);

    assert_eq!(info.ret, [1.to_bytes32()].concat());
    Ok(())
}

fn locals(module: &str) -> Result<()> {
    let bytecode = common::load(module, "locals")?;
    let info = EVM::run(&bytecode, &[]);

    assert_eq!(info.ret, [10.to_bytes32()].concat());
    Ok(())
}

impl_tests! {
    tests: [params, locals],
    modules: ["i32sub", "i64sub"]
}
