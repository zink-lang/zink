//! Storage example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::DoubleKeyMapping as _;

/// Counter with value type `i32`
#[zink::storage(i32, i32, i32)]
pub struct DoubleKeyMapping;

/// Set the mapping
#[zink::external]
pub fn mset(key1: i32, key2: i32, value: i32) {
    DoubleKeyMapping::set(key1, key2, value);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn storage_double_key_mapping() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract};

    let mut contract = Contract::search("dkmapping")?.compile()?;
    let mut evm = contract.deploy()?.commit(true);

    let key1 = 0x00;
    let key2 = 0x01;
    let value: i32 = 0x42;

    // set value to storage
    let calldata = contract.encode(&[
        b"mset(int32,int32,int32)".to_vec(),
        value.to_bytes32().to_vec(),
        key2.to_bytes32().to_vec(),
        key1.to_bytes32().to_vec(),
    ])?;
    let info = evm.calldata(&calldata).call(contract.address)?;
    assert!(info.ret.is_empty());

    // verify result with database
    let storage_key = zint::keccak256(&[[0; 32], [0; 32], 1.to_bytes32()].concat());
    assert_eq!(
        evm.storage(contract.address, storage_key)?,
        value.to_bytes32(),
    );

    // get value from storage
    let calldata = contract.encode(&[
        b"double_key_mapping(int32,int32)".to_vec(),
        key2.to_bytes32().to_vec(),
        key1.to_bytes32().to_vec(),
    ])?;
    let info = evm.calldata(&calldata).call(contract.address)?;
    assert_eq!(info.ret, value.to_bytes32(), "{info:#?}",);
    Ok(())
}
