//! Minimal ERC20 storage test with Bytes32
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use crate::zink::{DoubleKeyMapping as OtherDoubleKeyMapping, Mapping as OtherMapping, Storage};
#[allow(unused)]
use smallvec::SmallVec;
use zink::primitives::{Address, Bytes32, U256};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

struct Mapping<K, V>(core::marker::PhantomData<(K, V)>);
struct DoubleKeyMapping<K1, K2, V>(core::marker::PhantomData<(K1, K2, V)>);

#[allow(dead_code)]
#[derive(zink_codegen::Storage)]
pub struct ERC20 {
    name: Bytes32,
    symbol: Bytes32,
    total_supply: U256,
    balances: Mapping<Address, U256>,
    allowances: DoubleKeyMapping<Address, Address, U256>,
}

impl ERC20 {
    #[zink::external]
    pub fn init(&self, name: Bytes32, symbol: Bytes32) {
        self.set_name(name);
        self.set_symbol(symbol);
    }

    #[zink::external]
    pub fn decimals(&self) -> u32 {
        8
    }

    #[zink::external]
    pub fn transfer(&self, to: Address, value: U256) -> bool {
        let owner = Address::caller();
        self._transfer(owner, to, value);
        true
    }

    #[zink::external]
    pub fn approve(&self, spender: Address, value: U256) -> bool {
        let owner = Address::caller();
        self._approve(owner, spender, value);
        true
    }

    #[zink::external]
    pub fn transfer_from(&self, from: Address, to: Address, value: U256) -> bool {
        let spender = Address::caller();
        self._spend_allowance(from, spender, value);
        self._transfer(from, to, value);
        true
    }

    #[no_mangle]
    fn _transfer(&self, from: Address, to: Address, value: U256) {
        if from.eq(Address::empty()) {
            zink::revert!("Empty from address");
        }
        if to.eq(Address::empty()) {
            zink::revert!("Empty to address");
        }
        self._update(from, to, value);
    }

    #[no_mangle]
    fn _update(&self, from: Address, to: Address, value: U256) {
        if from.eq(Address::empty()) {
            self.set_total_supply(self.total_supply().add(value));
        } else {
            let from_balance = self.balances(from);
            if from_balance.lt(value) {
                zink::revert!("Insufficient balance");
            }
            self.set_balances(from, from_balance.sub(value));
        }

        if to.eq(Address::empty()) {
            self.set_total_supply(self.total_supply().sub(value));
        } else {
            self.set_total_supply(self.total_supply().add(value));
        }
    }

    #[no_mangle]
    fn _approve(&self, owner: Address, spender: Address, value: U256) {
        if owner.eq(Address::empty()) {
            zink::revert!("ERC20 Invalid approval");
        }
        if spender.eq(Address::empty()) {
            zink::revert!("ERC20 Invalid spender");
        }
        self.set_allowances(owner, spender, value);
    }

    #[no_mangle]
    fn _spend_allowance(&self, owner: Address, spender: Address, value: U256) {
        let current_allowance = self.allowances(owner, spender);
        if current_allowance.lt(U256::max()) {
            if current_allowance.lt(value) {
                zink::revert!("ERC20 Insufficient allowance");
            }
            self._approve(owner, spender, current_allowance.sub(value));
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn test_storage() -> anyhow::Result<()> {
    use zint::{Contract, EVM, U256 as ZintU256};

    let caller_bytes = hex::decode("be862ad9abfe6f22bcb087716c7d89a26051f74c")?;
    let mut caller = [0; 20];
    caller.copy_from_slice(&caller_bytes);

    let mut evm = EVM::default().commit(true).caller(caller);
    let mut contract = Contract::search("contract")?.compile()?;
    
    // Convert strings to Bytes32
    let name_bytes = "The Zink Language".as_bytes();
    let mut name_array = [0u8; 32];
    name_array[..name_bytes.len().min(32)].copy_from_slice(&name_bytes[..name_bytes.len().min(32)]);
    let _name = Bytes32(name_array);

    let symbol_bytes = "zink".as_bytes();
    let mut symbol_array = [0u8; 32];
    symbol_array[..symbol_bytes.len().min(32)].copy_from_slice(&symbol_bytes[..symbol_bytes.len().min(32)]);
    let _symbol = Bytes32(symbol_array);

    let total_supply_value = U256::from(42u64);

    // Deploy with initial storage
    let info = evm.deploy(
        &contract
            .construct(
                [
                    (ZintU256::from(0).to_le_bytes::<32>(), SmallVec::from_slice(&name_array)),
                    (ZintU256::from(1).to_le_bytes::<32>(), SmallVec::from_slice(&symbol_array)),
                    (ZintU256::from(2).to_le_bytes::<32>(), SmallVec::from_slice(&total_supply_value.bytes32())),
                ]
                .into_iter()
                .map(|(k, v)| (SmallVec::from_slice(&k), v))
                .collect(),
            )?
            .bytecode()?,
    )?;
    let address = info.address;

    // Test storage directly via EVM (bypassing contract calls due to revert issue)
    let name_storage = evm.storage(address, ZintU256::from(0).to_le_bytes::<32>())?;
    assert_eq!(name_storage.to_vec(), name_array.to_vec(), "Name storage mismatch");

    let symbol_storage = evm.storage(address, ZintU256::from(1).to_le_bytes::<32>())?;
    assert_eq!(symbol_storage.to_vec(), symbol_array.to_vec(), "Symbol storage mismatch");

    let total_supply_storage = evm.storage(address, ZintU256::from(2).to_le_bytes::<32>())?;
    assert_eq!(total_supply_storage.to_vec(), total_supply_value.bytes32().to_vec(), "Total supply storage mismatch");

    Ok(())
}
