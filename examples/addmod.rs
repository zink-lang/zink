//! Addmod example for i64, i32, u64, u32.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;
use zink::primitives::{numeric::Numeric, U256};

#[zink::external]
pub fn addmod_i32(a: i32, b: i32, n: i32) -> i32 {
    a.addmod(b, n)
}

#[zink::external]
pub fn addmod_i64(a: i64, b: i64, n: i64) -> i64 {
    a.addmod(b, n)
}

#[zink::external]
pub fn addmod_u32(a: u32, b: u32, n: u32) -> u32 {
    a.addmod(b, n)
}

#[zink::external]
pub fn addmod_u64(a: u64, b: u64, n: u64) -> u64 {
    a.addmod(b, n)
}

#[zink::external]
pub fn addmod_U256(a: U256, b: U256, n: U256) -> U256 {
    a.addmod(b, n)
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

// FIXME: this test should pass, fix it to learn how to
// add opcodes to the compiler
#[ignore]
#[test]
fn test() -> anyhow::Result<()> {
    use zint::{Bytes32 as _, Contract};

    // Test for i32
    let mut contract_i32 = Contract::search("addmod_i32")?.compile()?;
    let info_i32 = contract_i32.execute([
        "addmod_i32(uint32,uint32,uint32)".as_bytes(),
        &3i32.to_bytes32(),
        &5i32.to_bytes32(),
        &7i32.to_bytes32(),
    ])?;
    assert_eq!(info_i32.ret, 1i32.to_bytes32());

    // Test for i64
    let mut contract_i64 = Contract::search("addmod_i64")?.compile()?;
    let info_i64 = contract_i64.execute([
        "addmod_i64(int64,int64,int64)".as_bytes(),
        &3i64.to_bytes32(),
        &5i64.to_bytes32(),
        &7i64.to_bytes32(),
    ])?;
    assert_eq!(info_i64.ret, 1i64.to_bytes32());

    // Test for u32
    let mut contract_u32 = Contract::search("addmod_u32")?.compile()?;
    let info_u32 = contract_u32.execute([
        "addmod_u32(uint32,uint32,uint32)".as_bytes(),
        &3u32.to_bytes32(),
        &5u32.to_bytes32(),
        &7u32.to_bytes32(),
    ])?;
    assert_eq!(info_u32.ret, 1u32.to_bytes32());

    // Test for u64
    let mut contract_u64 = Contract::search("addmod_u64")?.compile()?;
    let info_u64 = contract_u64.execute([
        "addmod_u64(uint64,uint64,uint64)".as_bytes(),
        &3u64.to_bytes32(),
        &5u64.to_bytes32(),
        &7u64.to_bytes32(),
    ])?;
    assert_eq!(info_u64.ret, 1u64.to_bytes32());

    // Test for U256
    let mut contract_u256 = Contract::search("addmod_U256")?.compile()?;
    let info_u256 = contract_u256.execute([
        "addmod_U256(uint256,uint256,uint256)".as_bytes(),
        &U256::from(3).to_bytes32(),
        &U256::from(5).to_bytes32(),
        &U256::from(7).to_bytes32(),
    ])?;
    assert_eq!(info_u256.ret, U256::from(1).to_bytes32());

    Ok(())
}
