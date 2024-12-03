//! Storage example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::{primitives::Bytes16, Storage};

/// Counter with value type `Bytes16`
#[zink::storage(Bytes16)]
pub struct Bytes;

/// set value to the storage.
#[zink::external]
pub fn set(value: Bytes16) {
    Bytes::set(value);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn value() -> anyhow::Result<()> {
    use zink::Asm;
    use zint::Contract;

    let mut contract = Contract::search("bytes")?.compile()?;
    let new_storage = [8u8; 16];
    let mut evm = contract.deploy()?.commit(true);

    evm.calldata(&contract.encode(&[b"set(bytes)".to_vec(), new_storage.to_vec()])?)
        .call(contract.address)?;

    assert_eq!(
        evm.storage(contract.address, [0; 32])?,
        Bytes16(new_storage).bytes32(),
    );

    Ok(())
}
