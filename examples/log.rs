#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::{primitives::{Address, U256}, Event};

/// Example events for demonstration
#[derive(Event)]
enum ERC20Events {
    Transfer(Address, Address, U256), 
    Approval(Address, Address, U256), 
}

#[zink::external]
pub fn log_examples(from: Address, to: Address, value: U256) {
    let transfer = ERC20Events::Transfer(from, to, value); 
    transfer;

    let approval = ERC20Events::Approval(from, to, value); 
    approval;
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}