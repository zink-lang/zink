//! Constructor example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::Storage;

#[zink::storage]
pub type Name = u32;

#[zink::storage]
pub type Symbol = u32;

#[zink::storage]
pub type TotalSupply = u32;

/// Set value to the storage.
#[zink::constructor]
pub fn constructor(name: u32, symbol: u32) {
    Name::set(name);
    Symbol::set(symbol);
}

/// Get value from the storage.
#[zink::external]
pub fn decimals() -> u32 {
    0
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn deploy() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract, EVM};

    let contract = Contract::search("erc20")?.compile()?;

    let mut evm = EVM::default();
    let mut info = evm.deploy(&contract.bytecode())?;

    info = evm
        .calldata(&contract.encode(&["decimals()"])?)
        .call(info.address)?;

    assert_eq!(info.ret, 0.to_bytes32());
    Ok(())
}
