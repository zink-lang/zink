//! Example templates for initializing Zink projects.
/// Structure representing a project example.
pub struct Example {
    pub lib_rs: &'static str,
    pub readme: &'static str,
}

/// Default "addition" example.
pub const ADDITION: Example = Example {
    lib_rs: r#"
//! ${name}
#![no_std]
#[cfg(not(test))]
extern crate zink;

/// Adds two numbers together.
#[no_mangle]
pub extern "C" fn addition(x: u64, y: u64) -> u64 {
    x + y
}
"#,
    readme: r#"
# ${name}

> An EVM contract written in Rust with [The Zink Project][zink].

## Getting Started

```
cargo install zinkup
elko build
ls target/zink/${name}.bin
```

[zink]: https://github.com/zink-lang/zink
"#,
};

/// ERC20 example based on constructor.rs.
pub const ERC20: Example = Example {
    lib_rs: r#"
//! ${name}
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
    
/// Initialize the contract with name and symbol.
#[zink::external]
pub fn init(name: String32, symbol: String32) {
    Name::set(name);
    Symbol::set(symbol);
}
    
/// Get the number of decimals (fixed at 8).
#[zink::external]
pub fn decimals() -> u32 {
    8
}
    
/// Transfer tokens from caller to another address.
#[zink::external]
pub fn transfer(to: Address, value: U256) -> bool {
    let owner = Address::caller();
    _transfer(owner, to, value);
    true
}
    
/// Approve a spender to transfer tokens on behalf of the caller.
#[zink::external]
pub fn approve(spender: Address, value: U256) -> bool {
    let owner = Address::caller();
    _approve(owner, spender, value);
    true
}
    
/// Transfer tokens from one address to another using an allowance.
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
    _update(from, to, value);
}
    
#[no_mangle]
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
        _approve(owner, spender, current_allowance.sub(value));
    }
}
    
#[cfg(not(target_arch = "wasm32"))]
fn main() {}
"#,
    readme: r#"
# ${name}

> An EVM contract written in Rust with [The Zink Project][zink].

## Getting Started

```
cargo install zinkup
elko build
ls target/zink/${name}.bin
```

[zink]: https://github.com/zink-lang/zink
"#,
};
