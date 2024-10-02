//! Bytes example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::{primitives::Address, Storage};

/// Counter with value type `i32`
#[zink::storage(Address)]
pub struct Owner;

/// set value to the storage.
#[zink::external]
pub fn is_owner(owner: Address) -> bool {
    Owner::get() == owner
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn test_owner() -> anyhow::Result<()> {
    use zint::Contract;

    let mut contract = Contract::search("owner")?.compile()?;

    Ok(())
}
