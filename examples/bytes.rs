//! Storage example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::{primitives::Bytes16, Storage};

/// Counter with value type `i32`
#[zink::storage(Bytes16)]
pub struct Bytes;

/// set value to the storage.
#[zink::external]
pub fn set(value: Bytes16) {
    Bytes::set(value);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[ignore]
#[test]
fn value() -> anyhow::Result<()> {
    // TODO: see `./owner.rs`
    Ok(())
}
