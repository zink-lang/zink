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

#[zink::external]
pub fn spend_allowance(spender: Address, value: U256) -> bool {
    let owner = Address::caller();
    let current_allowance = Allowance::get(owner, spender);
    if current_allowance.lt(U256::max()) {
        if current_allowance.lt(value) {
            zink::revert!("ERC20 Insufficient allowance");
        }

        _approve(owner, spender, current_allowance.sub(value))
    }
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

    let mut evm = EVM::default().commit(true).caller([1; 20]);
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

    let allowance = evm.storage(
        address,
        Allowance::storage_key(Address(evm.caller), Address(spender)),
    )?;
    assert_eq!(value.to_bytes32(), allowance);

    // TODO: #273
    //
    // error: invalid jump while there are to composed functions
    //
    // This could be caused by the `caller()` on stack is not consumed correctly
    //
    // // spend allowance
    // let half_value = 21;
    // let info = evm
    //     .calldata(&contract.encode(&[
    //         b"spend_allowance(address,uint256)".to_vec(),
    //         spender.to_bytes32().to_vec(),
    //         half_value.to_bytes32().to_vec(),
    //     ])?)
    //     .call(address)?;
    // println!("{info:?}");
    // assert_eq!(info.ret, true.to_bytes32());
    // let allowance = evm.storage(
    //     address,
    //     Allowance::storage_key(Address(evm.caller), Address(spender)),
    // )?;
    // assert_eq!(half_value.to_bytes32(), allowance);
    Ok(())
}
