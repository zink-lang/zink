//! Storage example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::Mapping as _;

/// Counter with value type `i32`
#[zink::storage(i32, i32)]
pub struct Mapping;

/// Set the mapping
#[zink::external]
pub fn mset(key: i32, value: i32) {
    Mapping::set(key, value);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn storage_mapping() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract};

    let mut contract = Contract::search("mapping")?.compile()?;
    let mut evm = contract.deploy()?.commit(true);

    let key = 0x00;
    let value: i32 = 0x42;

    // set value to storage
    let calldata = contract.encode(&[
        b"mset(int32,int32)".to_vec(),
        value.to_bytes32().to_vec(),
        key.to_bytes32().to_vec(),
    ])?;
    let info = evm.calldata(&calldata).call(contract.address)?;
    assert!(info.ret.is_empty());

    tracing::debug!("{info:?}");
    // verify result with database
    let storage_key = zint::keccak256(&[0; 0x40]);
    assert_eq!(
        evm.storage(contract.address, storage_key)?,
        value.to_bytes32(),
    );

    // get value from storage
    let calldata = contract.encode(&[b"mapping(int32)".to_vec(), key.to_bytes32().to_vec()])?;
    let info = evm.calldata(&calldata).call(contract.address)?;
    assert_eq!(info.ret, value.to_bytes32(), "{info:#?}",);
    Ok(())
}
