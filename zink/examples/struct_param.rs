//! Struct parameter example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

#[zink::external]
pub fn sum(pair: (i32, i32)) -> i32 {
    pair.0 + pair.1
}

#[zink::external]
pub fn sum_with_extra(pair: (i32, i32), extra: i32) -> i32 {
    pair.0 + pair.1 + extra
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn struct_parameter() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract};

    let mut contract = Contract::search("struct_param")?.compile()?;
    let info = contract.execute([
        b"sum((int32,int32))".to_vec(),
        1i32.to_bytes32().to_vec(),
        2i32.to_bytes32().to_vec(),
    ])?;

    assert_eq!(info.ret, 3i32.to_bytes32());

    let info = contract.execute([
        b"sum_with_extra((int32,int32),int32)".to_vec(),
        1i32.to_bytes32().to_vec(),
        2i32.to_bytes32().to_vec(),
        4i32.to_bytes32().to_vec(),
    ])?;

    assert_eq!(info.ret, 7i32.to_bytes32());
    Ok(())
}
