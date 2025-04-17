//! Example for Block and Transaction Properties.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;
use zink::primitives::{properties, Address, Bytes32};

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
pub fn blobhash(index: u64) -> Bytes32 {
    properties::blobhash(index)
}

#[zink::external]
pub fn basefee() -> u64 {
    properties::basefee()
}

#[zink::external]
pub fn gasprice() -> u64 {
    properties::gasprice()
}

#[zink::external]
pub fn blobbasefee() -> u64 {
    properties::blobbasefee()
}

#[zink::external]
pub fn gaslimit() -> Bytes32 {
    properties::gaslimit()
}

#[zink::external]
pub fn coinbase() -> Address {
    properties::coinbase()
}

#[zink::external]
pub fn prevrandao() -> Bytes32 {
    properties::prevrandao()
}

#[zink::external]
pub fn timestamp() -> u64 {
    properties::timestamp()
}

#[zink::external]
pub fn gasleft() -> Bytes32 {
    properties::gasleft()
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use zint::{Bytes32, Contract, EVM};

    fn hash_to_bytes32(data: &str) -> [u8; 32] {
        let hash_bytes = hex::decode(data).unwrap();
        let mut hash = [0; 32];
        hash.copy_from_slice(&hash_bytes);
        hash
    }

    fn u64_to_bytes32(value: u64) -> Vec<u8> {
        let bytes = value.to_be_bytes();
        let mut bytes32 = [0; 32];
        bytes32[32 - bytes.len()..].copy_from_slice(&bytes);
        bytes32.to_vec()
    }

    #[test]
    fn test_block_properties() -> anyhow::Result<()> {
        let data = "29045A592007D0C246EF02C2223570DA9522D0CF0F73282C79A1BC8F0BB2C238";
        let mut evm = EVM::default()
            .chain_id(1)
            .block_number(599423555)
            .block_hash(599423545, hash_to_bytes32(data))
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
        assert_eq!(info.ret, hash_to_bytes32(data), "{info:?}");
        Ok(())
    }

    #[test]
    fn test_blob_properties() -> anyhow::Result<()> {
        let blobhash =
            hash_to_bytes32("0100000000000000000000000000000000000000000000000000000000000001");
        let mut evm = EVM::default().blob_hashes(vec![blobhash]).commit(true);
        let contract = Contract::search("properties")?.compile()?;
        let info = evm.deploy(&contract.bytecode()?)?;
        let address = info.address;

        let info = evm
            .calldata(&contract.encode(["blobhash(uint64)".as_bytes(), &u64_to_bytes32(0)])?)
            .call(address)?;
        assert_eq!(info.ret, blobhash, "{info:?}");

        let info = evm
            .calldata(&contract.encode(["blobhash(uint64)".as_bytes(), &u64_to_bytes32(1)])?)
            .call(address)?;
        assert_eq!(info.ret, 0u64.to_bytes32(), "{info:?}");
        Ok(())
    }

    #[test]
    fn test_fee_properties() -> anyhow::Result<()> {
        let mut evm = EVM::default()
            .basefee(100, 200)
            .blob_basefee(50)
            .commit(true);
        let contract = Contract::search("properties")?.compile()?;
        let info = evm.deploy(&contract.bytecode()?)?;
        let address = info.address;

        let info = evm
            .calldata(&contract.encode(["basefee()".as_bytes()])?)
            .call(address)?;
        assert_eq!(info.ret, 100u64.to_bytes32(), "{info:?}");

        let info = evm
            .calldata(&contract.encode(["gasprice()".as_bytes()])?)
            .call(address)?;
        assert_eq!(info.ret, 200u64.to_bytes32(), "{info:?}");

        let info = evm
            .calldata(&contract.encode(["blobbasefee()".as_bytes()])?)
            .call(address)?;
        assert_eq!(info.ret, evm.get_blob_basefee(), "{info:?}");

        let info = evm
            .calldata(&contract.encode(["gaslimit()".as_bytes()])?)
            .call(address)?;
        assert_eq!(info.ret, [255; 32], "{info:?}");
        Ok(())
    }

    #[test]
    fn test_coinbase() -> anyhow::Result<()> {
        let data = "29045A592007D0C246EF02C2223570DA9522D0CF0F73282C79A1BC8F0BB2C238";
        let mut evm = EVM::default()
            .coinbase([1; 20])
            .prevrandao(hash_to_bytes32(data))
            .timestamp(27)
            .commit(false);
        let contract = Contract::search("properties")?.compile()?;
        let info = evm.deploy(&contract.bytecode()?)?;
        let address = info.address;

        let info = evm
            .calldata(&contract.encode(["coinbase()".as_bytes()])?)
            .call(address)?;
        assert_eq!(info.ret, [1; 20].to_bytes32(), "{info:?}");

        let info = evm
            .calldata(&contract.encode(["prevrandao()".as_bytes()])?)
            .call(address)?;
        assert_eq!(info.ret, hash_to_bytes32(data), "{info:?}");

        let info = evm
            .calldata(&contract.encode(["timestamp()".as_bytes()])?)
            .call(address)?;
        assert_eq!(info.ret, 27u64.to_bytes32(), "{info:?}");
        Ok(())
    }

    #[test]
    fn test_gas_left() -> anyhow::Result<()> {
        let contract = Contract::search("properties")?.compile()?;

        let mut evm1 = EVM::default().tx_gas_limit(50000).commit(true);
        let info1 = evm1.deploy(&contract.bytecode()?)?;
        let info1 = evm1
            .calldata(&contract.encode(["gasleft()".as_bytes()])?)
            .call(info1.address)?;
        let gasleft1 = u64::from_be_bytes(info1.ret[24..].try_into().unwrap());
        let gas1 = 50000 - gasleft1;

        let mut evm2 = EVM::default().tx_gas_limit(70000).commit(true);
        let info2 = evm2.deploy(&contract.bytecode()?)?;
        let info2 = evm2
            .calldata(&contract.encode(["gasleft()".as_bytes()])?)
            .call(info2.address)?;
        let gasleft2 = u64::from_be_bytes(info2.ret[24..].try_into().unwrap());
        let gas2 = 70000 - gasleft2;
        assert_eq!(gas1, gas2);
        Ok(())
    }
}
