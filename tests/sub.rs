//! Addition tests for the zink compiler.
#![cfg(test)]

use anyhow::Result;
use filetests::{impl_tests, Test};
use zint::{Bytes32, Contract};

fn params(module: &str) -> Result<()> {
    let mut contract = Contract::new(Test::load(module, "params")?.wasm)
        .without_dispatcher()
        .compile()?;
    let info = contract.execute([2, 1])?;

    assert_eq!(info.ret, [1.to_bytes32()].concat());
    Ok(())
}

fn locals(module: &str) -> Result<()> {
    let mut contract = Contract::new(Test::load(module, "locals")?.wasm)
        .without_dispatcher()
        .compile()?;
    let info = contract.execute::<()>([])?;

    assert_eq!(info.ret, [10.to_bytes32()].concat());
    Ok(())
}

impl_tests! {
    tests: [params, locals],
    modules: ["i32sub", "i64sub"]
}
