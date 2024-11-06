//! Storage example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::Storage;

/// Counter with value type `i32`
#[zink::storage(i32)]
pub struct Counter;

/// set value to the storage.
#[zink::external]
pub fn set(value: i32) {
    Counter::set(value);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn value() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract, U256};

    let mut contract = Contract::search("storage")?.compile()?;
    let value: i32 = 42;

    {
        let info = contract.execute(&[b"set(int32)".to_vec(), value.to_bytes32().to_vec()])?;
        assert!(info.ret.is_empty());
        assert_eq!(
            info.storage.get(&U256::from_le_bytes(Counter::STORAGE_KEY)),
            Some(&U256::from(value))
        );
    }

    {
        let info = contract.execute(&["counter()"])?;
        assert_eq!(info.ret, 0.to_bytes32());
    }

    Ok(())
}
