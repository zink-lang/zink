//! Bytes example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::{primitives::Address, Storage};

/// Counter with value type `i32`
#[zink::storage(Address)]
pub struct Owner;

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
