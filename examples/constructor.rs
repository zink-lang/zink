//! Constructor example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::Storage;

/// It gets expanded to 'Counter' struct
/// that implements zink::Storage trait
/// (::set and ::get)
///
/// Storage key is taken based on macro order
/// (e.g this macro is first and only in this project,
/// so it will take 0x0 contract storage key)
#[zink::storage]
pub type Counter = i32;

/// Get value from the storage.
#[zink::external]
pub fn get() -> i32 {
    Counter::get() + 1
}

/// Set value to the storage.
#[zink::constructor]
pub fn constructor(value: i32) {
    Counter::set(value);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[cfg(test)]
mod tests {
    use zint::{ethers::abi::Abi, Contract, Ethers};

    #[tokio::test]
    #[allow(deprecated)]
    async fn constructor() -> zint::Result<()> {
        let api = Ethers::anvil()?;
        let contract = Contract::search("constructor")?
            .constructor(true)
            .compile()?;

        // let info = contract.execute([])?;
        // println!("{:?}", info);

        let abi: Abi = Abi::load(&*contract.json_abi()?.as_bytes())
            .map_err(|e| anyhow::anyhow!("Failed to load abi {e}"))?;
        let factory = api.factory(abi, contract.bytecode)?;
        let contract = factory.deploy(())?.legacy().send().await?;
        let r = contract.method::<(), i32>("get", ())?.call().await?;

        assert_eq!(r, 1);
        Ok(())
    }
}
