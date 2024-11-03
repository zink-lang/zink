//! Constructor example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::{
    primitives::{Address, String32, U256},
    Mapping, Storage,
};

#[zink::storage(String32)]
pub struct Name;

#[zink::storage(String32)]
pub struct Symbol;

#[zink::storage(U256)]
pub struct TotalSupply;

#[zink::storage(Address, U256)]
pub struct Balances;

/// Get value from the storage.
#[zink::external]
pub fn init(name: String32, symbol: String32) {
    Name::set(name);
    Symbol::set(symbol);
}

/// Get value from the storage.
#[zink::external]
pub fn decimals() -> u32 {
    8
}

fn _transfer(_from: Address, _to: Address) {
    // TODO: check and reverts
}

fn _update(_from: Address) {}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn deploy() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract, EVM};

    let mut contract = Contract::search("erc20")?.compile()?;

    let mut evm = EVM::default();
    let name = "The Zink Language";
    let symbol = "zink";

    // 1. deploy
    let info = evm.deploy(
        &contract
            .construct(
                [
                    (Name::STORAGE_KEY.to_bytes32().into(), name.to_vec().into()),
                    (
                        Symbol::STORAGE_KEY.to_bytes32().into(),
                        symbol.to_vec().into(),
                    ),
                    (
                        TotalSupply::STORAGE_KEY.to_bytes32().into(),
                        vec![42].try_into()?,
                    ),
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
    assert_eq!(info.ret, name.to_bytes32());

    // 3. get symbol
    let info = evm
        .calldata(&contract.encode(&[b"symbol()".to_vec()])?)
        .call(address)?;
    assert_eq!(info.ret, symbol.to_bytes32());

    // 3. get total supply
    let info = evm
        .calldata(&contract.encode(&[b"total_supply()".to_vec()])?)
        .call(address)?;
    assert_eq!(info.ret, 42u64.to_bytes32());

    // 4. check decimals
    let info = evm
        .calldata(&contract.encode(&[b"decimals()".to_vec()])?)
        .call(address)?;
    assert_eq!(info.ret, 8u64.to_bytes32());

    Ok(())
}
