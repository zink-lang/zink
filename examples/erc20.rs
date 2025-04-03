//! Constructor example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::{
    primitives::{Address, String32, U256},
    DoubleKeyMapping, Event, Mapping, Storage,
};
use crate::zink::Asm;

#[derive(Event)]
pub enum DebugEvent {
    Caller(Address),
    FromAddr(Address),
    Balance(U256),
    Insufficient(U256, U256),
    TestLog(U256),
}

#[zink::external]
pub fn test_log(value: U256) {
    DebugEvent::TestLog(value).emit();
}

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
    let owner = Address::caller();
    DebugEvent::Caller(owner).emit();
    DebugEvent::TestLog(value).emit();
    _transfer(owner, to, value);
    true
}

#[zink::external]
pub fn approve(spender: Address, value: U256) -> bool {
    let owner = Address::caller();
    _approve(owner, spender, value);
    true
}

#[zink::external]
pub fn transfer_from(from: Address, to: Address, value: U256) -> bool {
    let spender = Address::caller();
    _spend_allowance(from, spender, value);
    _transfer(from, to, value);
    true
}

#[no_mangle]
fn _transfer(from: Address, to: Address, value: U256) {
    if from.eq(Address::empty()) {
        zink::revert!("Empty from address");
    }

    if to.eq(Address::empty()) {
        zink::revert!("Empty to address");
    }

    _update(from, to, value)
}

#[no_mangle]
fn _update(from: Address, to: Address, value: U256) {
    if from.eq(Address::empty()) {
        TotalSupply::set(TotalSupply::get().add(value));
    } else {
        let from_balance = Balances::get(from);
        DebugEvent::FromAddr(from).emit();
        DebugEvent::Balance(from_balance).emit();
        if from_balance.lt(value) {
            DebugEvent::Insufficient(from_balance, value).emit();
            zink::revert!("Insufficient balance");
        }
        
        Balances::set(from, from_balance.sub(value));
        DebugEvent::TestLog(from_balance).emit();
    }

    if to.eq(Address::empty()) {
        TotalSupply::set(TotalSupply::get().sub(value));
    } else {
        let to_balance = Balances::get(to);
        Balances::set(to, to_balance.add(value));
    }
}

#[no_mangle]
fn _mint(account: Address, value: U256) {
    if account.eq(Address::empty()) {
        zink::revert!("ERC20 invalid receiver");
    }

    _update(Address::empty(), account, value)
}

#[no_mangle]
fn _burn(account: Address, value: U256) {
    if account.eq(Address::empty()) {
        zink::revert!("ERC20 invalid sender");
    }

    _update(account, Address::empty(), value)
}

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

#[no_mangle]
fn _spend_allowance(owner: Address, spender: Address, value: U256) {
    let current_allowance = Allowance::get(owner, spender);
    if current_allowance.lt(U256::max()) {
        if current_allowance.lt(value) {
            zink::revert!("ERC20 Insufficient allowance");
        }

        _approve(owner, spender, current_allowance.sub(value))
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn deploy() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract, EVM};

    let caller_bytes = hex::decode("be862ad9abfe6f22bcb087716c7d89a26051f74c")?;
    let spender = [42; 20];
    let mut caller = [0; 20];
    caller.copy_from_slice(&caller_bytes);

    let mut evm = EVM::default().commit(true).caller(caller);
    let mut contract = Contract::search("erc20")?.compile()?;
    let name = "The Zink Language";
    let symbol = "zink";
    let value = 42;
    let half_value = 21;

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
                    (
                        Balances::storage_key(Address::from(caller)).into(),
                        vec![42].try_into()?,
                    ),
                ]
                .into_iter()
                .collect(),
            )?
            .bytecode()?,
    )?;
    let address = info.address;

    // 2. get static data
    {
        // 2.1. get name
        let info = evm
            .calldata(&contract.encode(&[b"name()".to_vec()])?)
            .call(address)?;
        assert_eq!(info.ret, name.to_bytes32(), "{info:?}");

        // 2.2. get symbol
        let info = evm
            .calldata(&contract.encode(&[b"symbol()".to_vec()])?)
            .call(address)?;
        assert_eq!(info.ret, symbol.to_bytes32(), "{info:?}");

        // 2.3. get total supply
        let info = evm
            .calldata(&contract.encode(&[b"total_supply()".to_vec()])?)
            .call(address)?;
        assert_eq!(info.ret, 42u64.to_bytes32(), "{info:?}");

        // 2.4. check decimals
        let info = evm
            .calldata(&contract.encode(&[b"decimals()".to_vec()])?)
            .call(address)?;
        assert_eq!(info.ret, 8u64.to_bytes32(), "{info:?}");

        // 2.5. check balance of the caller
        let balance = evm.storage(address, Balances::storage_key(Address::from(caller)))?;
        assert_eq!(value.to_bytes32(), balance);
    }

    // 3. check approval
    {
        // 3.1. approve
        let info = evm
            .calldata(&contract.encode(&[
                b"approve(address,uint256)".to_vec(),
                spender.to_bytes32().to_vec(),
                value.to_bytes32().to_vec(),
            ])?)
            .call(address)?;
        assert_eq!(info.ret, true.to_bytes32(), "{info:?}");

        let allowance = evm.storage(
            address,
            Allowance::storage_key(Address::from(evm.caller), Address::from(spender)),
        )?;
        assert_eq!(value.to_bytes32(), allowance);

        // 3.2. check approval results
        let info = evm
            .calldata(&contract.encode(&[
                b"allowance(address,address)".to_vec(),
                evm.caller.to_bytes32().to_vec(),
                spender.to_bytes32().to_vec(),
            ])?)
            .call(address)?;
        assert_eq!(info.ret, allowance);
    }

    // Test log emission
    let info = contract.execute(&[
        b"test_log(uint256)".to_vec(),
        U256::from(42).0.bytes32().to_vec(),
    ])?;
    println!("Test log result: {info:?}");

    // 4. check transfer
    {
        // 4.1. verify balance of the caller
        let info = evm
            .calldata(&contract.encode(&[
                b"balances(address)".to_vec(),
                evm.caller.to_bytes32().to_vec(),
            ])?)
            .call(address)?;
        assert_eq!(info.ret, value.to_bytes32(), "{info:?}");

        // Debug: Check balance immediately before transfer
        let balance_before = evm.storage(address, Balances::storage_key(Address::from(caller)))?;
        println!("Balance in storage before transfer: {:?}", balance_before);
        let info = evm
            .calldata(&contract.encode(&[
                b"balances(address)".to_vec(),
                evm.caller.to_bytes32().to_vec(),
            ])?)
            .call(address)?;
        println!("Balance via balances() before transfer: {:?}", info.ret);

        //  TODO: see br_balance.rs (#287)
        // 4.2. check transfer
        evm = evm.commit(false);
        println!(
            "EVM storage after commit(false): {:?}",
            evm.storage(address, Balances::storage_key(Address::from(caller)))?
        );
        let info = evm
            .calldata(&contract.encode(&[
                b"transfer(address,uint256)".to_vec(),
                spender.to_bytes32().to_vec(),
                half_value.to_bytes32().to_vec(),
            ])?)
            .call(address)?;
        println!("Transfer result: {info:?}");
        println!("Logs: {:?}", info.logs);
        assert_eq!(info.ret, true.to_bytes32(), "{info:?}");
    }

    Ok(())
}
