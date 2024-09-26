//! Constructor example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::Storage;

#[zink::storage(u32)]
struct Name;

#[zink::storage(u32)]
struct Symbol;

#[zink::storage(u32)]
struct TotalSupply;

// #[zink::storage(i32 => i32)]
// struct Balances;

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
pub fn name() -> u32 {
    Name::get()
}

#[zink::external]
pub fn symbol() -> u32 {
    Symbol::get()
}

#[zink::external]
pub fn total_supply() -> u32 {
    TotalSupply::get()
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn deploy() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract, EVM};

    let mut contract = Contract::search("erc20")?.compile()?;

    let mut evm = EVM::default();
    let mut calldata: Vec<u8> = Default::default();
    calldata.extend_from_slice(&42.to_bytes32());
    calldata.extend_from_slice(&42.to_bytes32());

    // 1. deploy
    let info = evm.deploy(
        &contract
            .construct(
                [
                    (vec![0].try_into()?, vec![42].try_into()?),
                    (vec![1].try_into()?, vec![42].try_into()?),
                    (vec![2].try_into()?, vec![42].try_into()?),
                ]
                .into_iter()
                .collect(),
            )?
            .bytecode()?,
    )?;
    let address = info.address;

    // 2. get name
    let info = evm
        .calldata(&contract.encode(&[b"name()".to_vec()])?)
        .call(address)?;
    assert_eq!(info.ret, 42u64.to_bytes32());

    // 3. get symbol
    let info = evm
        .calldata(&contract.encode(&[b"symbol()".to_vec()])?)
        .call(address)?;
    assert_eq!(info.ret, 42u64.to_bytes32());

    // 3. get symbol
    let info = evm
        .calldata(&contract.encode(&[b"total_supply()".to_vec()])?)
        .call(address)?;
    assert_eq!(info.ret, 42u64.to_bytes32());

    Ok(())
}
