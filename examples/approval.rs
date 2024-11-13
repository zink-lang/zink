#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::{
    primitives::{Address, U256},
    DoubleKeyMapping,
};

#[zink::storage(Address, Address, U256)]
pub struct Allowance;

#[zink::external]
pub fn approve(spender: Address, value: U256) -> bool {
    let owner = Address::caller();
    _approve(owner, spender, value);
    true
}

// NOTE: issue #272
//
// #[no_mangle] here is required otherwise the inner functions could
// not get the passing variables correctly.
#[no_mangle]
fn _approve(owner: Address, spender: Address, value: U256) {
    if owner.eq(Address::empty()) {
        zink::revert!("ERC20 Invalid approval");
    }

    if spender.eq(Address::empty()) {
        zink::revert!("ERC20 Invalid spender");
    }

    Allowance::set(owner, spender, value);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn test_approval() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract, EVM};

    let mut evm = EVM::default().commit(true);
    let contract = Contract::search("approval")?.compile()?;
    let info = evm.deploy(&contract.bytecode()?)?;
    let address = info.address;

    let value = 42;
    let spender = [42; 20];
    let info = evm
        .calldata(&contract.encode(&[
            b"approve(address,uint256)".to_vec(),
            spender.to_bytes32().to_vec(),
            value.to_bytes32().to_vec(),
        ])?)
        .call(address)?;
    assert_eq!(info.ret, true.to_bytes32());

    let stored_value = evm.storage(
        address,
        Allowance::storage_key(Address(evm.caller), Address(spender)),
    )?;
    assert_eq!(value.to_bytes32(), stored_value);
    Ok(())
}
