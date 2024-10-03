//! Bytes example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::{primitives::Address, Storage};

/// Contract owner storage
#[zink::storage(Address)]
pub struct Owner;

/// set owner
#[zink::external]
pub fn is_owner(owner: Address) -> bool {
    Owner::get() == owner
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[ignore]
#[test]
fn test_owner() -> anyhow::Result<()> {
    use zint::{Contract, EVM};
    let mut contract = Contract::search("owner")?.compile()?;

    let mut evm = EVM::default();
    let not_owner = [8; 20];
    let mut info = evm.deploy(
        &contract
            .construct(
                [(vec![0].try_into()?, not_owner.to_vec().try_into()?)]
                    .into_iter()
                    .collect(),
            )?
            .bytecode()?,
    )?;

    println!("{info:?}");

    info = evm
        .calldata(&contract.encode(&[b"is_owner(address)".to_vec(), [0; 32].to_vec()])?)
        .call(info.address)?;

    println!("{info:?}");

    Ok(())
}
