//! Addition tests for the zink compiler.
#![cfg(test)]

use anyhow::Result;
use zinkc::Compiler;

mod common;

#[test]
fn i32_add() -> Result<()> {
    let wasm = common::load("i32add", "params")?;
    let bytecode = Compiler::compile(&wasm)?;

    assert_eq!(hex::encode(bytecode), "6000356020350160005260206000f3");
    Ok(())
}
