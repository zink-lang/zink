//! Example of revert
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

/// check if the passing address is owner
#[zink::external]
pub fn revert() {
    zink::revert!("revert works")
}

/// check if the passing address is owner
#[zink::external]
pub fn assert() {
    zink::assert!(false, "assert works");
}

#[test]
fn test_revert() -> anyhow::Result<()> {
    use zint::Contract;
    let mut contract = Contract::search("revert")?.compile()?;

    let info = contract.execute(["revert()".as_bytes()])?;
    assert_eq!(info.revert, Some("revert works".into()));

    let info = contract.execute(["assert()".as_bytes()])?;
    assert_eq!(info.revert, Some("assert works".into()));
    Ok(())
}
