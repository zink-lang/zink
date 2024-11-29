#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;
use zink::{storage, Storage};

#[storage(i32)]
struct Balance;

#[zink::external]
fn check_and_update(value: i32) -> bool {
    let current = Balance::get();

    // This mimics the ERC20 balance check
    if current < value {
        return false;
    }

    Balance::set(current - value);
    true
}

// TODO: identify if the problem is caused by control flow of incorrect opcode mapping.
#[ignore]
#[test]
fn test_balance_check() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract, EVM};

    let mut evm = EVM::default();
    let mut contract = Contract::search("br_balance")?.compile()?;

    // Initialize with balance of 42
    let info = evm.deploy(
        &contract
            .construct(
                [(
                    Balance::STORAGE_KEY.to_bytes32().into(),
                    vec![42].try_into()?,
                )]
                .into_iter()
                .collect(),
            )?
            .bytecode()?,
    )?;

    // Try to transfer 21 (should succeed)
    let info = evm
        .calldata(&contract.encode(&[
            b"check_and_update(int32)".to_vec(),
            21i32.to_bytes32().to_vec(),
        ])?)
        .call(info.address)?;

    assert_eq!(info.ret, true.to_bytes32());

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
