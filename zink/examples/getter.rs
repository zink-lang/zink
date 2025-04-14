//! Simple getter example to debug dispatcher
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::primitives::Bytes32;
use zink::Storage;

#[allow(dead_code)]
#[derive(zink_codegen::Storage)]
pub struct Getter {
    value: Bytes32,
}

impl Getter {
    #[zink::external]
    pub fn get_value(&self) -> Bytes32 {
        self.value()
    }

    #[zink::external]
    pub fn init(&self, value: Bytes32) {
        self.set_value(value);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn test_getter() -> anyhow::Result<()> {
    #[allow(unused)]
    use smallvec::SmallVec;
    use zint::{Contract, EVM, U256 as ZintU256};

    let caller_bytes = hex::decode("be862ad9abfe6f22bcb087716c7d89a26051f74c")?;
    let mut caller = [0; 20];
    caller.copy_from_slice(&caller_bytes);

    let mut evm = EVM::default().commit(true).caller(caller);
    let mut contract = Contract::search("getter")?.compile()?;

    let value_bytes = "TestValue".as_bytes();
    let mut value_array = [0u8; 32];
    value_array[..value_bytes.len().min(32)]
        .copy_from_slice(&value_bytes[..value_bytes.len().min(32)]);
    let _value = Bytes32(value_array);

    // Deploy with initial value
    let info = evm.deploy(
        &contract
            .construct(
                [(
                    ZintU256::from(0).to_le_bytes::<32>(),
                    SmallVec::from_slice(&value_array),
                )]
                .into_iter()
                .map(|(k, v)| (SmallVec::from_slice(&k), v))
                .collect(),
            )?
            .bytecode()?,
    )?;
    let address = info.address;

    // Test direct storage read
    let stored_value = evm.storage(address, ZintU256::from(0).to_le_bytes::<32>())?;
    assert_eq!(
        stored_value.to_vec(),
        value_array.to_vec(),
        "Storage mismatch"
    );

    // Test runtime getter
    let info = evm
        .calldata(&contract.encode(&[b"get_value()".to_vec()])?)
        .call(address)?;
    assert_eq!(info.ret, value_array.to_vec(), "Getter failed: {:?}", info);

    Ok(())
}
