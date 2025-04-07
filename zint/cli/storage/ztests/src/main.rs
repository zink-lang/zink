
use zink::primitives::Bytes32;
use zint::{Contract, EVM};

fn main() {
    println!("Run `cargo zint` to execute tests");
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use smallvec::SmallVec;
    use zint::U256 as ZintU256;

    #[test]
    fn test_storage() -> Result<()> {
        let caller_bytes = hex::decode("be862ad9abfe6f22bcb087716c7d89a26051f74c")?;
        let mut caller = [0; 20];
        caller.copy_from_slice(&caller_bytes);

        let mut evm = EVM::default().commit(true).caller(caller);
        let mut contract = Contract::search("storage_test")?.compile()?;

        let value_bytes = "TestValue".as_bytes();
        let mut value_array = [0u8; 32];
        value_array[..value_bytes.len().min(32)].copy_from_slice(&value_bytes[..value_bytes.len().min(32)]);
        let value = Bytes32(value_array);

        let info = evm.deploy(&contract.bytecode()?)?;
        let address = info.address;

        let info = evm.calldata(&contract.encode(&[b"getValue()".to_vec()])?).call(address)?;
        assert_eq!(info.ret, vec![0u8; 32], "Getter should return default 0 initially");

        Ok(())
    }
}
