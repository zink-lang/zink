//! Transient Storage example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::storage::TransientStorage;

/// Temporary counter with value type `i32` that resets after each transaction
#[zink::transient_storage(i32)]
pub struct TempCounter;

/// Set value to the transient storage.
#[zink::external]
pub fn set_temp(value: i32) {
    TempCounter::set(value);
}

/// Get value from the transient storage.
#[zink::external]
pub fn get_temp() -> i32 {
    TempCounter::get()
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn transient_value() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract, U256};

    let mut contract = Contract::search("transient_storage")?.compile()?;
    let value: i32 = 42;

    // First transaction: set value
    {
        let info = contract.execute(&[b"set_temp(int32)".to_vec(), value.to_bytes32().to_vec()])?;
        assert!(info.ret.is_empty());
        assert_eq!(
            info.transient_storage
                .get(&U256::from_le_bytes(TempCounter::STORAGE_KEY)),
            Some(&U256::from(value))
        );
        // Verify regular storage is untouched
        assert_eq!(
            info.storage
                .get(&U256::from_le_bytes(TempCounter::STORAGE_KEY)),
            None
        );
    }

    // Second transaction: value should be cleared
    {
        let info = contract.execute(&[b"get_temp()".to_vec()])?;
        assert_eq!(info.ret, 0.to_bytes32());
        // Verify transient storage was cleared
        assert_eq!(
            info.transient_storage
                .get(&U256::from_le_bytes(TempCounter::STORAGE_KEY)),
            None
        );
    }

    Ok(())
}

// Example with both storage types
#[test]
fn mixed_storage() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract, U256};

    let mut contract = Contract::search("transient_storage")?.compile()?;

    // Regular storage counter
    #[zink::storage(i32)]
    pub struct Counter;

    // Transient storage counter
    #[zink::transient_storage(i32)]
    pub struct TempCounter;

    let perm_value: i32 = 42;
    let temp_value: i32 = 84;

    // Set both values
    {
        let info = contract.execute(&[
            b"set_both(int32,int32)".to_vec(),
            perm_value.to_bytes32().to_vec(),
            temp_value.to_bytes32().to_vec(),
        ])?;

        // Verify both storages have correct values
        assert_eq!(
            info.storage.get(&U256::from_le_bytes(Counter::STORAGE_KEY)),
            Some(&U256::from(perm_value))
        );
        assert_eq!(
            info.transient_storage
                .get(&U256::from_le_bytes(TempCounter::STORAGE_KEY)),
            Some(&U256::from(temp_value))
        );
    }

    // Next transaction: only permanent value should remain
    {
        let info = contract.execute(&[b"get_both()".to_vec()])?;
        assert_eq!(
            info.storage.get(&U256::from_le_bytes(Counter::STORAGE_KEY)),
            Some(&U256::from(perm_value))
        );
        assert_eq!(
            info.transient_storage
                .get(&U256::from_le_bytes(TempCounter::STORAGE_KEY)),
            None
        );
    }

    Ok(())
}
