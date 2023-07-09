//! if condtion tests for the zink compiler.
#![cfg(test)]

use anyhow::Result;
use zinkc::Compiler;

mod common;

#[test]
fn singular() -> Result<()> {
    let wasm = common::load("if", "singular")?;
    let bytecode = Compiler::compile(&wasm)?;

    // assert_eq!(hex::encode(bytecode), "600060016000f3");
    Ok(())
}
