//! Example for Block and Transaction Properties.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;
use zink::primitives::{properties, Bytes32};

#[zink::external]
pub fn blockhash(block_number: u64) -> Bytes32 {
    properties::blockhash(block_number)
}

#[zink::external]
pub fn number() -> u64 {
    properties::number()
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn test_block_properties() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract, EVM};

    let mut evm = EVM::default().commit(true);
    let contract = Contract::search("properties")?.compile()?;
    let raw_info = evm.deploy(&contract.bytecode()?)?;

    let info = evm
        .calldata(&contract.encode(&[b"number()".to_vec()])?)
        .call(raw_info.address)?;
    assert_eq!(info.ret, 0u64.to_bytes32(), "{info:?}");

    let info = evm
        .calldata(&contract.encode(&[
            b"blockhash(uint64)".to_vec(),
            599423545u64.to_bytes32().to_vec(),
        ])?)
        .call(raw_info.address)?;
    assert_eq!(info.ret, 0u64.to_bytes32(), "{info:?}");

    Ok(())
}
