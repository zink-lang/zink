//! Addition example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]
extern crate zink;

/// Adds two numbers together.
#[zink::external]
pub fn addition(x: u64, y: u64) -> u64 {
    x + y
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn test() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract};
    let mut contract = Contract::search("addition")?.compile()?;

    let info = contract.execute([
        "addition(u64,u64)".as_bytes(),
        &1u64.to_bytes32(),
        &2u64.to_bytes32(),
    ])?;
    assert_eq!(info.ret, 3u64.to_bytes32());

    Ok(())
}
