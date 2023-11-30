//! Storage example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::Storage;

/// It gets expanded to 'Counter' struct
/// that implements zink::Storage trait
/// (::set and ::get)
///
/// Storage key is taken based on macro order
/// (e.g this macro is first and only in this project,
/// so it will take 0x0 contract storage key)
#[zink::storage]
pub type Counter = i32;

/// Set value to the storage and get it.
#[zink::external]
pub fn set_and_get(value: i32) -> i32 {
    Counter::set(value);
    Counter::get()
}

/// set value to the storage.
#[zink::external]
pub fn set(value: i32) {
    Counter::set(value);
}

/// Get value from the storage.
#[zink::external]
pub fn get() -> i32 {
    Counter::get()
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[cfg(test)]
mod tests {
    use zint::{
        ethers::abi::{Abi, Function, Param, ParamType, StateMutability},
        Bytes32, Contract, Contract, Ethers, InstructionResult, U256,
    };

    #[test]
    fn selector() -> anyhow::Result<()> {
        let mut contract = Contract::search("storage")?.compile()?;

        {
            let key = 0;
            let value: i32 = 42;
            let info = contract.execute(&[b"set(i32)".to_vec(), value.to_bytes32().to_vec()])?;
            assert!(info.ret.is_empty());
            assert_eq!(info.instr, InstructionResult::Return);
            assert_eq!(info.storage.get(&U256::from(key)), Some(&U256::from(value)));
        }

        {
            let info = contract.execute(&["get()"])?;
            assert_eq!(info.instr, InstructionResult::Return);
            assert_eq!(info.ret, 0.to_bytes32());
        }

        {
            let key = 0;
            let value = 42;
            let info =
                contract.execute(&[b"set_and_get(i32)".to_vec(), value.to_bytes32().to_vec()])?;
            assert_eq!(info.instr, InstructionResult::Return);
            assert_eq!(info.ret, value.to_bytes32());
            assert_eq!(info.storage.get(&U256::from(key)), Some(&U256::from(value)));
        }

        Ok(())
    }

    #[tokio::test]
    #[allow(deprecated)]
    async fn constructor() -> zint::Result<()> {
        let api = Ethers::anvil()?;
        let bytecode = Contract::search("storage")?
            .constructor(true)
            .compile()?
            .bytecode;

        // TODO: Generate ABI issue #47
        let mut abi: Abi = Default::default();
        abi.functions.insert(
            "get".into(),
            vec![Function {
                name: "get".into(),
                inputs: Default::default(),
                outputs: vec![Param {
                    name: Default::default(),
                    kind: ParamType::Int(32usize),
                    internal_type: None,
                }],
                constant: None,
                state_mutability: StateMutability::View,
            }],
        );

        let factory = api.factory(abi, bytecode)?;
        let contract = factory.deploy(())?.legacy().send().await?;
        let r = contract.method::<(), i32>("get", ())?.call().await?;

        assert_eq!(r, 0);
        Ok(())
    }
}
