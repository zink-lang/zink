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

// /// Set value to the storage.
// #[zink::constructor]
// pub fn constructor(name: u32, symbol: u32) {
//     Name::set(name);
//     Symbol::set(symbol);
// }

/// Get value from the storage.
#[zink::external]
pub fn init(name: u32, symbol: u32) {
    Name::set(name);
    Symbol::set(symbol);
}

/// Get value from the storage.
#[zink::external]
pub fn decimals() -> u32 {
    2
}

#[zink::external]
pub fn symbol() -> u32 {
    Symbol::get()
}

#[zink::external]
pub fn name() -> u32 {
    Name::get()
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[ignore]
#[test]
fn deploy() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract, EVM};

    let contract = Contract::search("erc20")?.compile()?;

    let mut evm = EVM::default();
    let mut calldata: Vec<u8> = Default::default();
    calldata.extend_from_slice(&42.to_bytes32());
    calldata.extend_from_slice(&42.to_bytes32());

    // 1. deploy
    let info = evm.deploy(&contract.bytecode()?)?;
    let address = info.address;

    // 2. init state
    evm.calldata(&contract.encode(&[
        b"init(uint32,uint32)".to_vec(),
        16u64.to_bytes32().to_vec(),
        42u64.to_bytes32().to_vec(),
    ])?)
    .call(address)?;

    // 3. get name
    let info = evm
        .calldata(&contract.encode(&[b"name()".to_vec()])?)
        .call(address)?;
    assert_eq!(info.ret, 16u64.to_bytes32());

    // 4. get symbol
    let info = evm
        .calldata(&contract.encode(&[b"symbol()".to_vec()])?)
        .call(address)?;
    assert_eq!(info.ret, 42u64.to_bytes32());

    Ok(())
}
