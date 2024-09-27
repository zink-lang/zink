//! Constructor example.
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
#[zink::storage(i32)]
pub struct Counter;

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn noop() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract, EVM};

    let contract = Contract::search("constructor")?.compile()?;

    // empty constructor
    let mut evm = EVM::default();
    let mut info = evm.deploy(&contract.bytecode()?)?;
    info = evm
        .calldata(&contract.encode(&["get()"])?)
        .call(info.address)?;

    assert_eq!(info.ret, 0.to_bytes32());
    Ok(())
}

#[test]
fn init_storage() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract, EVM};

    let mut contract = Contract::search("constructor")?.compile()?;
    let value = 0x42;

    // empty constructor
    let mut evm = EVM::default();
    let mut info = evm.deploy(
        &contract
            .construct(
                [(vec![0].try_into()?, vec![value].try_into()?)]
                    .into_iter()
                    .collect(),
            )?
            .bytecode()?,
    )?;
    info = evm
        .calldata(&contract.encode(&["counter()"])?)
        .call(info.address)?;

    assert_eq!(info.ret, value.to_bytes32());
    Ok(())
}
