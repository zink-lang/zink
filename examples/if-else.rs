//! if-else example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

/// Simple if-else condition
#[zink::external]
pub fn if_else(x: u64, y: u64) -> u64 {
    if x > y {
        x
    } else {
        y
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

// TODO: fix this test on `#166`
#[ignore]
#[test]
fn test() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract};
    let mut contract = Contract::search("if-else")?.compile()?;

    let info = contract.execute([
        "if_else(u64,u64)".as_bytes(),
        &1u64.to_bytes32(),
        &2u64.to_bytes32(),
    ])?;
    assert_eq!(info.ret, [2.to_bytes32()].concat());

    let info = contract.execute([
        "if_else(u64,u64)".as_bytes(),
        &2u64.to_bytes32(),
        &1u64.to_bytes32(),
    ])?;
    assert_eq!(info.ret, [2.to_bytes32()].concat());

    Ok(())
}
