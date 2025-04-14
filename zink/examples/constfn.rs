//! Storage example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

/// set value to the storage.
#[zink::external]
pub fn decimals() -> i32 {
    8
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn value() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract};

    let mut contract = Contract::search("constfn")?.compile()?;

    let decimals = 8.to_bytes32().to_vec();
    let info = contract.execute(&[b"decimals()".to_vec(), decimals.clone()])?;
    assert_eq!(info.ret, decimals);
    Ok(())
}
