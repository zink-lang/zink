#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::primitives::Address;

#[zink::external]
pub fn run_delegatecall() -> bool {
    unsafe { zink::asm::evm::delegatecall(50_000, Address::empty(), 0, 0, 0, 0) }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
