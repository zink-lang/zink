//! Transient Storage example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::storage::TransientStorage;
/// Temporary counter with value type `i32` that resets after each transaction
#[zink::transient_storage(i32)]
pub struct TempCounter;

/// Set and get value via the transient storage.
#[zink::external]
pub fn set_and_get_temp(value: i32) -> i32 {
    TempCounter::set(value);
    TempCounter::get()
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn transient_value() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract, U256};

    let mut contract = Contract::search("transient_storage")?.compile()?;
    let value: i32 = 42;

    {
        let info = contract.execute(&[
            b"set_and_get_temp(int32)".to_vec(),
            value.to_bytes32().to_vec(),
        ])?;
        assert!(info.ret.is_empty());
        assert_eq!(
            info.storage
                .get(&U256::from_le_bytes(TempCounter::STORAGE_KEY)),
            Some(&U256::from(value))
        );
    }

    Ok(())
}
