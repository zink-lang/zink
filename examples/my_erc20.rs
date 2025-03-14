#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

#[cfg(all(target_arch = "wasm32", feature = "wasm-alloc"))]
extern crate dlmalloc;
extern crate zink;
#[cfg(all(target_arch = "wasm32", feature = "wasm-alloc"))]
#[global_allocator]
static ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc; // Set as global allocator

#[cfg(target_arch = "wasm32")]
extern crate alloc; // Add alloc for wasm32
#[cfg(target_arch = "wasm32")]
use alloc::string::String;

use zink::{
    primitives::{Address, U256},
    DoubleKeyMapping, Mapping,
};

// Storage for balances
#[zink::storage(Address, U256)]
pub struct Balances;

// Storage for allowances
#[zink::storage(Address, Address, U256)]
pub struct Allowances;

#[zink::external]
pub fn name() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        String::from("ZinkToken")
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::string::String::from("ZinkToken")
    }
}

#[zink::external]
pub fn symbol() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        String::from("ZTK")
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::string::String::from("ZTK")
    }
}

#[zink::external]
pub fn decimals() -> u8 {
    18
}

#[zink::external]
pub fn balance_of(owner: Address) -> U256 {
    Balances::get(owner)
}

#[zink::external]
pub fn transfer(to: Address, value: U256) -> bool {
    let sender = Address::caller();
    if to.eq(Address::empty()) {
        zink::revert!("ERC20: Transfer to zero address");
    }
    let sender_balance = Balances::get(sender);
    if sender_balance.lt(value) {
        zink::revert!("ERC20: Insufficient balance");
    }
    Balances::set(sender, sender_balance.sub(value));
    let recipient_balance = Balances::get(to);
    Balances::set(to, recipient_balance.add(value));
    true
}

#[zink::external]
pub fn approve(spender: Address, value: U256) -> bool {
    let owner = Address::caller();
    _approve(owner, spender, value);
    true
}

#[zink::external]
pub fn allowance(owner: Address, spender: Address) -> U256 {
    Allowances::get(owner, spender)
}

#[zink::external]
pub fn spend_allowance(spender: Address, value: U256) -> bool {
    let owner = Address::caller();
    let current_allowance = Allowances::get(owner, spender);
    if current_allowance.lt(U256::max()) {
        if current_allowance.lt(value) {
            zink::revert!("ERC20: Insufficient allowance");
        }
        _approve(owner, spender, current_allowance.sub(value));
    }
    true
}

#[no_mangle]
fn _approve(owner: Address, spender: Address, value: U256) {
    if owner.eq(Address::empty()) {
        zink::revert!("ERC20: Invalid approval");
    }
    if spender.eq(Address::empty()) {
        zink::revert!("ERC20: Invalid spender");
    }
    Allowances::set(owner, spender, value);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn dummy_test() {
        // Minimal test to satisfy zinkc-filetests
        assert!(true);
    }
}
