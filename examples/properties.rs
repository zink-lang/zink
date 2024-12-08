//! Example for Block and Transaction Properties.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;
use zink::primitives::{properties, Bytes32};

#[zink::external]
pub fn number() -> u64 {
    properties::number()
}

#[zink::external]
pub fn blockhash(number: u64) -> Bytes32 {
    properties::blockhash(number)
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[cfg(test)]
mod tests {
    use zint::{Bytes32, Contract, EVM};

    fn get_block_attr() -> ([u8; 32], [u8; 32]) {
        let mut block_number = 599423545u64.to_bytes32();
        block_number.reverse();
        let hash_bytes =
            hex::decode("29045A592007D0C246EF02C2223570DA9522D0CF0F73282C79A1BC8F0BB2C238")
                .unwrap();
        let mut block_hash = [0; 32];
        block_hash.copy_from_slice(&hash_bytes);
        (block_number, block_hash)
    }

    #[test]
    fn test_block_properties() -> anyhow::Result<()> {
        let (block_number, block_hash) = get_block_attr();
        let mut evm = EVM::default()
            .block_number(block_number)
            .block_hash(block_hash)
            .commit(false);
        let contract = Contract::search("properties")?.compile()?;
        let address = evm.deploy(&contract.bytecode()?)?.address;

        let info = evm
            .calldata(&contract.encode(["number()".as_bytes()])?)
            .call(address)?;
        assert_eq!(info.ret, block_number, "{info:?}");

        let info = evm
            .calldata(&contract.encode(["blockhash(uint64)".as_bytes(), &block_number])?)
            .call(address)?;
        assert_eq!(info.ret, 0u64.to_bytes32(), "{info:?}");
        Ok(())
    }
}
