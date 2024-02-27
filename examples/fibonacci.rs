//! fibonacci example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

// for the panic handler.
extern crate zink;

/// Calculates the nth fibonacci number.
#[zink::external]
pub fn fib(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn test() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract};
    let mut contract = Contract::search("fibonacci")?.compile()?;
    let selector = "fib(uint64)".as_bytes();

    // x = 0
    let info = contract.execute([selector, &0u64.to_bytes32()])?;
    assert_eq!(0.to_bytes32().to_vec(), info.ret);

    // x = 1
    let info = contract.execute([selector, &1u64.to_bytes32()])?;
    assert_eq!(1.to_bytes32().to_vec(), info.ret);

    // x = 2
    let info = contract.execute([selector, &2u64.to_bytes32()])?;
    assert_eq!(1.to_bytes32().to_vec(), info.ret);

    // x = 3
    let info = contract.execute([selector, &3u64.to_bytes32()])?;
    assert_eq!(2.to_bytes32().to_vec(), info.ret);

    // x = 4
    let info = contract.execute([selector, &4u64.to_bytes32()])?;
    assert_eq!(3.to_bytes32().to_vec(), info.ret);

    // x = 5
    let info = contract.execute([selector, &5u64.to_bytes32()])?;
    assert_eq!(5.to_bytes32().to_vec(), info.ret);

    Ok(())
}
