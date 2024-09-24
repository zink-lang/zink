//! Storage example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::Mapping as _;

/// Counter with value type `i32`
#[zink::storage(i32 => i32)]
pub struct Mapping;

/// Set the mapping
#[zink::external]
pub fn mset(key: i32, value: i32) {
    Mapping::set(key, value);
}

/// Get from ampping
#[zink::external]
pub fn mget(key: i32) -> i32 {
    Mapping::get(key)
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[ignore]
#[test]
fn mapping() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract, U256};

    let mut contract = Contract::search("storage")?.compile()?;

    {
        let key = 0;
        let value: i32 = 42;
        let info = contract.execute(&[
            b"mset(int32,int32)".to_vec(),
            key.to_bytes32().to_vec(),
            value.to_bytes32().to_vec(),
        ])?;
        assert!(info.ret.is_empty());
        assert_eq!(info.storage.get(&U256::from(0)), Some(&U256::from(value)));
    }

    {
        let key = 0;
        let info = contract.execute(&[b"mget(int32)".to_vec(), key.to_bytes32().to_vec()])?;
        assert_eq!(info.ret, 0.to_bytes32());
    }

    Ok(())
}
