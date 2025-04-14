//! Bytes example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::{primitives::Address, Storage};

/// Contract owner storage
#[zink::storage(Address)]
pub struct Owner;

/// check if the passing address is owner
#[zink::external]
pub fn is_owner(owner: Address) -> bool {
    Owner::get().eq(owner)
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn test_owner() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract};
    let mut contract = Contract::search("owner")?.compile()?;
    let not_owner = [8; 20];
    let mut evm = contract
        .construct(
            [(vec![0].try_into()?, not_owner.to_vec().try_into()?)]
                .into_iter()
                .collect(),
        )?
        .deploy()?
        .commit(true);

    assert_eq!(
        evm.storage(contract.address, [0; 32])?,
        not_owner.to_bytes32(),
    );

    assert_eq!(
        evm.calldata(&contract.encode(&[b"is_owner(address)".to_vec(), [0; 32].to_vec()])?)
            .call(contract.address)?
            .ret,
        false.to_bytes32().to_vec()
    );

    assert_eq!(
        evm.calldata(&contract.encode(&[
            b"is_owner(address)".to_vec(),
            not_owner.to_bytes32().to_vec(),
        ])?)
        .call(contract.address)?
        .ret,
        true.to_bytes32().to_vec()
    );
    Ok(())
}
