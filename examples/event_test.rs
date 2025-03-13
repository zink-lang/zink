//! Event ABI generation test
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

#[allow(unused_imports)]
use zink::primitives::Bytes32;
use zink::primitives::{Address, U256};

#[derive(zink_codegen::Event)]
pub enum MyEvent {
    Transfer {
        #[indexed]
        from: Address,
        #[indexed]
        to: Address,
        value: U256,
    },
    #[anonymous]
    Approval {
        #[indexed]
        owner: Address,
        value: U256,
    },
}

#[zink::external]
pub fn emit_transfer(from: Address, to: Address, value: U256) {
    MyEvent::Transfer { from, to, value }.emit();
}

#[zink::external]
pub fn emit_approval(owner: Address, value: U256) {
    MyEvent::Approval { owner, value }.emit();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{self, Value};

    #[test]
    fn test_event_abi() -> anyhow::Result<()> {
        let abi_str = MyEvent::abi();
        let abi: Value = serde_json::from_str(abi_str)?;

        let events = abi.as_array().unwrap();
        assert_eq!(events.len(), 2);

        let transfer = &events[0];
        assert_eq!(transfer["name"].as_str().unwrap(), "MyEvent");
        assert_eq!(transfer["anonymous"].as_bool().unwrap(), false);
        let inputs = transfer["inputs"].as_array().unwrap();
        assert_eq!(inputs.len(), 3);
        assert_eq!(inputs[0]["name"].as_str().unwrap(), "from");
        assert_eq!(inputs[0]["type"].as_str().unwrap(), "address");
        assert_eq!(inputs[0]["indexed"].as_bool().unwrap(), true);
        assert_eq!(inputs[1]["name"].as_str().unwrap(), "to");
        assert_eq!(inputs[1]["type"].as_str().unwrap(), "address");
        assert_eq!(inputs[1]["indexed"].as_bool().unwrap(), true);
        assert_eq!(inputs[2]["name"].as_str().unwrap(), "value");
        assert_eq!(inputs[2]["type"].as_str().unwrap(), "uint256");
        assert_eq!(inputs[2]["indexed"].as_bool().unwrap(), false);

        let approval = &events[1];
        assert_eq!(approval["name"].as_str().unwrap(), "MyEvent");
        assert_eq!(approval["anonymous"].as_bool().unwrap(), true);
        let inputs = approval["inputs"].as_array().unwrap();
        assert_eq!(inputs.len(), 2);
        assert_eq!(inputs[0]["name"].as_str().unwrap(), "owner");
        assert_eq!(inputs[0]["type"].as_str().unwrap(), "address");
        assert_eq!(inputs[0]["indexed"].as_bool().unwrap(), true);
        assert_eq!(inputs[1]["name"].as_str().unwrap(), "value");
        assert_eq!(inputs[1]["type"].as_str().unwrap(), "uint256");
        assert_eq!(inputs[1]["indexed"].as_bool().unwrap(), false);

        Ok(())
    }

    #[test]
    fn test_event_emission_mock() -> anyhow::Result<()> {
        let from = Address::from([1; 20]);
        let to = Address::from([2; 20]);
        let value = U256::from(42u64);

        let event = MyEvent::Transfer { from, to, value };
        match event {
            MyEvent::Transfer { from, to, value: _ } => {
                let from_bytes = from.bytes32(); // Non-WASM: [u8; 32]
                let to_bytes = to.bytes32(); // Non-WASM: [u8; 32]
                assert_eq!(from_bytes.len(), 32);
                assert_eq!(to_bytes.len(), 32);
                assert_eq!(MyEvent::name(), b"MyEvent");
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}
