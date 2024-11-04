//! Constructor example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::{
    primitives::{Address, String32, U256},
    DoubleKeyMapping, Mapping, Storage,
};

#[zink::storage(String32)]
pub struct Name;

#[zink::storage(String32)]
pub struct Symbol;

#[zink::storage(U256)]
pub struct TotalSupply;

#[zink::storage(Address, U256)]
pub struct Balances;

#[zink::storage(Address, Address, U256)]
pub struct Allowance;

/// Get value from the storage.
#[zink::external]
pub fn init(name: String32, symbol: String32) {
    Name::set(name);
    Symbol::set(symbol);
}

/// Get value from the storage.
#[zink::external]
pub fn decimals() -> u32 {
    8
}

#[zink::external]
pub fn transfer(to: Address, value: U256) -> bool {
    let owner = unsafe { zink::ffi::evm::msg_sender() };
    _transfer(owner, to, value);
    true
}

#[zink::external]
pub fn approve(spender: Address, value: U256) -> bool {
    // TODO: wrap this in env
    let owner = unsafe { zink::ffi::evm::msg_sender() };
    _approve(owner, spender, value, false);
    true
}

#[zink::external]
pub fn transfer_from(from: Address, to: Address, value: U256) -> bool {
    let spender = unsafe { zink::ffi::evm::msg_sender() };
    _spend_allowance(from, spender, value);
    _transfer(from, to, value);
    true
}

fn _transfer(from: Address, to: Address, value: U256) {
    if from.eq(Address::empty()) {
        zink::revert!("Empty from address");
    }

    if to.eq(Address::empty()) {
        zink::revert!("Empty to address");
    }

    _update(from, to, value)
}

fn _update(from: Address, to: Address, value: U256) {
    if from.eq(Address::empty()) {
        TotalSupply::set(TotalSupply::get().add(value));
    } else {
        let from_balance = Balances::get(from);
        if from_balance.lt(value) {
            zink::revert!("Insufficient balance");
        }

        Balances::set(from, from_balance.sub(value));
    }

    if to.eq(Address::empty()) {
        TotalSupply::set(TotalSupply::get().sub(value));
    } else {
        TotalSupply::set(TotalSupply::get().add(value));
    }
}

fn _mint(account: Address, value: U256) {
    if account.eq(Address::empty()) {
        zink::revert!("ERC20 invalid receiver");
    }

    _update(Address::empty(), account, value)
}

fn _burn(account: Address, value: U256) {
    if account.eq(Address::empty()) {
        zink::revert!("ERC20 invalid sender");
    }

    _update(account, Address::empty(), value)
}

fn _approve(owner: Address, spender: Address, value: U256, _emit_event: bool) {
    if owner.eq(Address::empty()) {
        zink::revert!("ERC20 Invalid approval");
    }

    if spender.eq(Address::empty()) {
        zink::revert!("ERC20 Invalid spender");
    }

    Allowance::set(owner, spender, value);
}

fn _spend_allowance(owner: Address, spender: Address, value: U256) {
    let current_allowance = Allowance::get(owner, spender);
    if current_allowance.lt(U256::max()) {
        if current_allowance.lt(value) {
            zink::revert!("ERC20 Insufficient allowance");
        }

        _approve(owner, spender, current_allowance.sub(value), false)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[ignore]
#[test]
fn deploy() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract, EVM};

    let mut contract = Contract::search("erc20")?.compile()?;

    let mut evm = EVM::default();
    let name = "The Zink Language";
    let symbol = "zink";

    // 1. deploy
    let info = evm.deploy(
        &contract
            .construct(
                [
                    (Name::STORAGE_KEY.to_bytes32().into(), name.to_vec().into()),
                    (
                        Symbol::STORAGE_KEY.to_bytes32().into(),
                        symbol.to_vec().into(),
                    ),
                    (
                        TotalSupply::STORAGE_KEY.to_bytes32().into(),
                        vec![42].try_into()?,
                    ),
                ]
                .into_iter()
                .collect(),
            )?
            .bytecode()?,
    )?;
    let address = info.address;

    // 2. get name
    let info = evm
        .calldata(&contract.encode(&[b"name()".to_vec()])?)
        .call(address)?;
    assert_eq!(info.ret, name.to_bytes32());

    // 3. get symbol
    let info = evm
        .calldata(&contract.encode(&[b"symbol()".to_vec()])?)
        .call(address)?;
    assert_eq!(info.ret, symbol.to_bytes32());

    // 3. get total supply
    let info = evm
        .calldata(&contract.encode(&[b"total_supply()".to_vec()])?)
        .call(address)?;
    assert_eq!(info.ret, 42u64.to_bytes32());

    // 4. check decimals
    let info = evm
        .calldata(&contract.encode(&[b"decimals()".to_vec()])?)
        .call(address)?;
    assert_eq!(info.ret, 8u64.to_bytes32());

    Ok(())
}
