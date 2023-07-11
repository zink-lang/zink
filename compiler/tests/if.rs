//! if-else tests for the zink compiler.
#![cfg(test)]

use anyhow::Result;
use zinkc::Compiler;

mod common;

#[test]
fn basic() -> Result<()> {
    let wasm = common::load("if", "basic")?;
    let _bytecode = Compiler::compile(&wasm)?;

    // assert_eq!(hex::encode(bytecode), "6000356020350160005260206000f3");
    Ok(())
}
