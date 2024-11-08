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
    let owner = unsafe { zink::ffi::evm::caller() };
    _approve(owner, spender, value);
    true
}

fn _approve(owner: Address, spender: Address, value: U256) {
    // if owner.eq(Address::empty()) {
    //     zink::revert!("ERC20 Invalid approval");
    // }

    // if spender.eq(Address::empty()) {
    //     zink::revert!("ERC20 Invalid spender");
    // }

    Allowance::set(owner, spender, value);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn test_approval() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract, EVM};

    let mut evm = EVM::default(); //.commit(true);
    let contract = Contract::search("approval")?.compile()?;
    let info = evm.deploy(&contract.bytecode()?)?;
    let address = info.address;

    println!("contract address: 0x{}", hex::encode(address));
    let info = evm
        .calldata(&contract.encode(&[
            b"approve(address,uint256)".to_vec(),
            [0; 20].to_bytes32().to_vec(),
            42.to_bytes32().to_vec(),
        ])?)
        .call(address)?;
    println!("{info:?}");
    // assert_eq!(info.ret, true.to_bytes32());
    //
    // let info = evm
    //     .calldata(&contract.encode(&[
    //         b"allowance(address,address)".to_vec(),
    //         [0; 20].to_bytes32().to_vec(),
    //         [0; 20].to_bytes32().to_vec(),
    //     ])?)
    //     .call(address)?;
    // println!("{info:?}");
    Ok(())
}
