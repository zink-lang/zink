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

/// Get value from the storage.
#[zink::external]
pub fn get() -> i32 {
    Counter::get() + 1
}

/// Set value to the storage.
#[zink::constructor]
pub fn constructor(value: i32) {
    Counter::set(value);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[cfg(test)]
mod tests {
    use zint::{
        ethers::abi::{Abi, Function, Param, ParamType, StateMutability},
        Contract, Ethers,
    };

    #[tokio::test]
    #[allow(deprecated)]
    async fn constructor() -> zint::Result<()> {
        let api = Ethers::anvil()?;
        let bytecode = Contract::search("constructor")?
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

        assert_eq!(r, 1);
        Ok(())
    }
}
