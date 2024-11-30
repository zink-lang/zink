// //! storage tests
// #![cfg(test)]

// use anyhow::Result;
// use filetests::Test;
// use zint::{keccak256, Bytes32, Contract, EVM, U256};

// #[test]
// fn store() -> Result<()> {
//     let mut contract = Contract::from(Test::STORAGE_STORE).pure().compile()?;

//     let key = 0u64;
//     let value = 42u64;
//     let info = contract.execute([value])?;
//     assert!(info.ret.is_empty());
//     assert_eq!(info.storage.get(&U256::from(key)), Some(&U256::from(value)));

//     Ok(())
// }

// #[test]
// fn load() -> Result<()> {
//     let mut contract = Contract::from(Test::STORAGE_LOAD).pure().compile()?;

//     let value = 42;
//     let info = contract.execute([value])?;
//     assert_eq!(info.ret, value.to_bytes32());

//     Ok(())
// }

// #[test]
// fn basic() -> Result<()> {
//     let mut contract = Contract::from(Test::STORAGE_BASIC).pure().compile()?;

//     let value = 42;
//     let info = contract.execute([value])?;
//     assert_eq!(info.ret, 42.to_bytes32());

//     Ok(())
// }

// #[test]
// fn transient_store() -> Result<()> {
//     let mut contract = Contract::from(Test::TRANSIENT_STORAGE_STORE).pure().compile()?;

//     let key = 0u64;
//     let value = 42u64;
//     let info = contract.execute([value])?;
//     assert!(info.ret.is_empty());
//     assert_eq!(info.transient_storage.get(&U256::from(key)), Some(&U256::from(value)));
//     // Verify regular storage is untouched
//     assert_eq!(info.storage.get(&U256::from(key)), None);

//     Ok(())
// }

// #[test]
// fn transient_load() -> Result<()> {
//     let mut contract = Contract::from(Test::TRANSIENT_STORAGE_LOAD).pure().compile()?;

//     let value = 42;
//     let info = contract.execute([value])?;
//     assert_eq!(info.ret, value.to_bytes32());
//     // Verify we can't load this from regular storage
//     assert_eq!(info.storage.get(&U256::from(0)), None);

//     Ok(())
// }

// #[test]
// fn transient_lifecycle() -> Result<()> {
//     let mut contract = Contract::from(Test::TRANSIENT_STORAGE_LIFECYCLE).pure().compile()?;

//     // First transaction: store value
//     let value = 42u64;
//     let info = contract.execute([value])?;
//     assert!(info.ret.is_empty());
//     assert_eq!(info.transient_storage.get(&U256::from(0)), Some(&U256::from(value)));

//     // Second transaction: value should be cleared
//     let info = contract.execute([])?;
//     assert_eq!(info.transient_storage.get(&U256::from(0)), None);

//     Ok(())
// }

// #[test]
// fn transient_mapping() -> Result<()> {
//     use opcodes::Cancun;
//     use zint::Bytes32;

//     zint::setup_logger();

//     let hashing: Vec<u8> = vec![
//         // storage value
//         Cancun::PUSH1,
//         Cancun::Data(0x42),
//         // Load storage slot
//         //
//         // write index to memory
//         Cancun::PUSH0,
//         Cancun::PUSH0,
//         Cancun::MSTORE8,
//         // write key to memory
//         Cancun::PUSH0,
//         Cancun::PUSH1,
//         Cancun::Data(0x01),
//         Cancun::MSTORE,
//         // hash key
//         Cancun::PUSH1,
//         Cancun::Data(0x20),
//         Cancun::PUSH0,
//         Cancun::KECCAK256,
//         // write to transient storage
//         Cancun::TSTORE,
//         // Load from transient storage
//         //
//         // write index to memory
//         Cancun::PUSH0,
//         Cancun::PUSH0,
//         Cancun::MSTORE8,
//         // write key to memory
//         Cancun::PUSH0,
//         Cancun::PUSH1,
//         Cancun::Data(0x01),
//         Cancun::MSTORE,
//         // hash key
//         Cancun::PUSH1,
//         Cancun::Data(0x20),
//         Cancun::PUSH0,
//         Cancun::KECCAK256,
//         // load from transient storage to stack
//         Cancun::TLOAD,
//         // write to memory
//         Cancun::PUSH0,
//         Cancun::MSTORE,
//         // return
//         Cancun::PUSH1,
//         Cancun::Data(0x20),
//         Cancun::PUSH0,
//         Cancun::RETURN,
//     ]
//     .into_iter()
//     .map(Into::into)
//     .collect();

//     let info = EVM::interp(&hashing, &[])?;
//     tracing::debug!("bytecode: {}", hex::encode(&hashing));

//     let key = keccak256(&[0; 0x20]);
//     assert_eq!(
//         info.transient_storage.get(&U256::from_be_bytes(key)),
//         Some(&U256::from_be_bytes(0x42.to_bytes32())),
//         "{info:#?}"
//     );
//     // Verify regular storage wasn't affected
//     assert_eq!(info.storage.get(&U256::from_be_bytes(key)), None);
//     assert_eq!(0x42.to_bytes32().to_vec(), info.ret);
//     Ok(())
// }

// #[test]
// fn mixed_storage() -> Result<()> {
//     let mut contract = Contract::from(Test::MIXED_STORAGE).pure().compile()?;

//     let perm_value = 42u64;
//     let trans_value = 84u64;
//     let info = contract.execute([perm_value, trans_value])?;
    
//     // Verify both storages contain their respective values
//     assert_eq!(info.storage.get(&U256::from(0)), Some(&U256::from(perm_value)));
//     assert_eq!(info.transient_storage.get(&U256::from(0)), Some(&U256::from(trans_value)));
    
//     Ok(())
// }