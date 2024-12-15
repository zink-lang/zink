#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

use zink::{
    primitives::{Address, U256},
    Error,
};

extern crate zink;

// Define custom errors
#[derive(Error)]
pub enum ContractError {
    Unauthorized,
    InsufficientBalance(U256, U256),
    InvalidAddress(Address),
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[zink::external]
pub fn simple_revert() {
    zink::revert!("simple revert message");
}

#[zink::external]
pub fn assert_example() {
    zink::assert!(false, "assertion failed");
}

#[zink::external]
pub fn custom_error() {
    ContractError::Unauthorized;
}

#[zink::external]
pub fn insufficient_balance(amount: U256) {
    let balance = U256::empty();
    ContractError::InsufficientBalance(balance, amount);
}

#[zink::external]
pub fn check_address(addr: Address) {
    ContractError::InvalidAddress(addr);
}

#[cfg(test)]
mod tests {
    use super::*;
    use zint::Contract;

    // #[test]
    // fn test_errors() -> anyhow::Result<()> {
    //     let mut contract = Contract::search("errors")?.compile()?;

    //     // Test string revert
    //     let info = contract.execute(["simple_revert()".as_bytes()])?;
    //     assert_eq!(info.revert, Some("simple revert message".into()));

    //     // Test assertion
    //     let info = contract.execute(["assert_example()".as_bytes()])?;
    //     assert_eq!(info.revert, Some("assertion failed".into()));

    //     // Test custom error
    //     let info = contract.execute(["custom_error()".as_bytes()])?;
    //     assert!(info.revert.is_some());
    //     // In practice, you'd decode the custom error from the revert data

    //     // Test custom error with data
    //     let info = contract.execute(["insufficient_balance(uint256)".as_bytes(), &[100u8; 32]])?;
    //     assert!(info.revert.is_some());
    //     // You'd decode the balance values from the revert data

    //     Ok(())
    // }
}
