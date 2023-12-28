//! Addition tests for the zink compiler.
#![cfg(test)]

use anyhow::Result;
use filetests::{impl_tests, Test};
use zint::{Bytes32, Contract};

fn params(module: &str) -> Result<()> {
    let mut contract = Contract::new(Test::load(module, "params")?.wasm)
        .pure()
        .compile()?;

    // add(1, 2)
    let info = contract.execute([1, 2])?;
    assert_eq!(info.ret, [3.to_bytes32()].concat());
    Ok(())
}

fn locals(module: &str) -> Result<()> {
    let mut contract = Contract::new(Test::load(module, "locals")?.wasm)
        .pure()
        .compile()?;
    let info = contract.execute::<()>([])?;
    assert_eq!(info.ret, [30.to_bytes32()].concat());
    Ok(())
}

fn tee(module: &str) -> Result<()> {
    let mut contract = Contract::new(Test::load(module, "tee")?.wasm)
        .pure()
        .compile()?;
    let info = contract.execute::<()>([])?;
    assert_eq!(info.ret, [30.to_bytes32()].concat());
    Ok(())
}

impl_tests! {
    tests: [params, locals, tee],
    modules: ["i32add", "i64add"]
}
