//! Example for Block and Transaction Properties.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;
use zink::primitives::{properties, Bytes32};

#[zink::external]
pub fn chainid() -> u64 {
    properties::chainid()
}

#[zink::external]
pub fn number() -> u64 {
    properties::number()
}

#[zink::external]
pub fn blockhash(number: u64) -> Bytes32 {
    properties::blockhash(number)
}

#[zink::external]
pub fn gasleft() -> u64 {
    properties::gas()
}

#[zink::external]
pub fn gaslimit() -> u64 {
    properties::gaslimit()
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[cfg(test)]
mod tests {
    use zint::{Bytes32, Contract, EVM};

    fn get_block_hash() -> [u8; 32] {
        let hash_bytes =
            hex::decode("29045A592007D0C246EF02C2223570DA9522D0CF0F73282C79A1BC8F0BB2C238")
                .unwrap();
        let mut block_hash = [0; 32];
        block_hash.copy_from_slice(&hash_bytes);
        block_hash
    }

    fn u64_to_bytes32(value: u64) -> Vec<u8> {
        let bytes = value.to_be_bytes();
        let mut bytes32 = [0; 32];
        bytes32[32 - bytes.len()..].copy_from_slice(&bytes);
        bytes32.to_vec()
    }

    #[test]
    fn test_block_properties() -> anyhow::Result<()> {
        let mut evm = EVM::default()
            .chain_id(1)
            .block_number(599423555)
            .block_hash(599423545, get_block_hash())
            .commit(true);
        let contract = Contract::search("properties")?.compile()?;
        let info = evm.deploy(&contract.bytecode()?)?;
        let address = info.address;

        let info = evm
            .calldata(&contract.encode(["chainid()".as_bytes()])?)
            .call(address)?;
        assert_eq!(info.ret, 1u64.to_bytes32(), "{info:?}");

        let info = evm
            .calldata(&contract.encode(["number()".as_bytes()])?)
            .call(address)?;
        assert_eq!(info.ret, u64_to_bytes32(599423555), "{info:?}");

        let info = evm
            .calldata(
                &contract.encode(["blockhash(uint64)".as_bytes(), &u64_to_bytes32(599423545)])?,
            )
            .call(address)?;
        assert_eq!(info.ret, get_block_hash(), "{info:?}");
        Ok(())
    }
}
