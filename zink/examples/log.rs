#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::{primitives::U256, Event};

#[derive(Event)]
pub enum MyEvent {
    /// Event with one topic
    Topic1(U256),
    /// Event with two topics
    Topic2(U256, U256),
    /// Event with three topics
    Topic3(U256, U256, U256),
    /// Event with four topics
    Topic4(U256, U256, U256, U256),
}

/// Test log0
#[zink::external]
pub fn test_log0() {
    MyEvent::emit_name();
}

/// Test log1
#[zink::external]
pub fn test_log1(value: U256) {
    MyEvent::Topic1(value).emit();
}

/// Test log2
#[zink::external]
pub fn test_log2(value1: U256, value2: U256) {
    MyEvent::Topic2(value1, value2).emit();
}

/// Test log3
#[zink::external]
pub fn test_log3(value1: U256, value2: U256, value3: U256) {
    MyEvent::Topic3(value1, value2, value3).emit();
}

/// Test log4
#[zink::external]
pub fn test_log4(value1: U256, value2: U256, value3: U256, value4: U256) {
    MyEvent::Topic4(value1, value2, value3, value4).emit();
}

#[cfg(test)]
mod tests {
    use zink::Value;
    use zint::{Bytes32, Contract};

    #[test]
    fn test_events() {
        let mut contract = Contract::search("log")
            .unwrap()
            .compile()
            .expect("failed to compile");

        let name = b"MyEvent";
        let value1: i32 = 1;
        let value2: i32 = 2;
        let value3: i32 = 3;
        let value4: i32 = 4;

        {
            // Test log0
            let info = contract.execute(&[b"test_log0()".to_vec()]).unwrap();
            assert!(!info.logs.is_empty());
            assert_eq!(
                info.logs[0].data.data.to_vec(),
                name.to_vec().to_bytes32().to_vec()
            );

            // Test log1
            let info = contract
                .execute(&[b"test_log1(uint256)".to_vec(), value1.bytes32().to_vec()])
                .expect("failed to execute test_log1");
            assert!(!info.logs.is_empty());
            assert_eq!(
                info.logs[0].data.data.to_vec(),
                name.to_vec().to_bytes32().to_vec()
            );
            assert_eq!(info.logs[0].topics()[0].to_vec(), value1.bytes32().to_vec());

            // Test log2
            let info = contract
                .execute(&[
                    b"test_log2(uint256,uint256)".to_vec(),
                    value1.bytes32().to_vec(),
                    value2.bytes32().to_vec(),
                ])
                .unwrap();
            assert!(!info.logs.is_empty());
            assert_eq!(info.logs[0].topics()[1].to_vec(), value1.bytes32().to_vec());
            assert_eq!(info.logs[0].topics()[0].to_vec(), value2.bytes32().to_vec());

            let info = contract
                .execute(&[
                    b"test_log3(uint256,uint256,uint256)".to_vec(),
                    value1.bytes32().to_vec(),
                    value2.bytes32().to_vec(),
                    value3.bytes32().to_vec(),
                ])
                .unwrap();
            assert!(!info.logs.is_empty());
            assert_eq!(info.logs[0].topics()[2].to_vec(), value1.bytes32().to_vec());
            assert_eq!(info.logs[0].topics()[1].to_vec(), value2.bytes32().to_vec());
            assert_eq!(info.logs[0].topics()[0].to_vec(), value3.bytes32().to_vec());

            let info = contract
                .execute(&[
                    b"test_log4(uint256,uint256,uint256,uint256)".to_vec(),
                    value1.bytes32().to_vec(),
                    value2.bytes32().to_vec(),
                    value3.bytes32().to_vec(),
                    value4.bytes32().to_vec(),
                ])
                .unwrap();
            assert!(!info.logs.is_empty());
            assert_eq!(info.logs[0].topics()[3].to_vec(), value1.bytes32().to_vec());
            assert_eq!(info.logs[0].topics()[2].to_vec(), value2.bytes32().to_vec());
            assert_eq!(info.logs[0].topics()[1].to_vec(), value3.bytes32().to_vec());
            assert_eq!(info.logs[0].topics()[0].to_vec(), value4.bytes32().to_vec());
        }
    }

    #[test]
    fn test_log_abi() {
        let contract = Contract::search("log")
            .unwrap()
            .compile()
            .expect("failed to compile");
        let abi = contract.artifact.abi;

        assert_eq!(abi.len(), 5);

        let test_log0 = abi.iter().find(|a| a.name == "test_log0").unwrap();
        assert_eq!(test_log0.inputs.len(), 0);

        let test_log1 = abi.iter().find(|a| a.name == "test_log1").unwrap();
        assert_eq!(test_log1.inputs.len(), 1);
        assert_eq!(test_log1.inputs[0].ty.to_string(), "uint256");

        let test_log2 = abi.iter().find(|a| a.name == "test_log2").unwrap();
        assert_eq!(test_log2.inputs.len(), 2);
        assert_eq!(test_log2.inputs[0].ty.to_string(), "uint256");
        assert_eq!(test_log2.inputs[1].ty.to_string(), "uint256");

        let test_log3 = abi.iter().find(|a| a.name == "test_log3").unwrap();
        assert_eq!(test_log3.inputs.len(), 3);
        assert_eq!(test_log3.inputs[0].ty.to_string(), "uint256");
        assert_eq!(test_log3.inputs[1].ty.to_string(), "uint256");
        assert_eq!(test_log3.inputs[2].ty.to_string(), "uint256");

        let test_log4 = abi.iter().find(|a| a.name == "test_log4").unwrap();
        assert_eq!(test_log4.inputs.len(), 4);
        assert_eq!(test_log4.inputs[0].ty.to_string(), "uint256");
        assert_eq!(test_log4.inputs[1].ty.to_string(), "uint256");
        assert_eq!(test_log4.inputs[2].ty.to_string(), "uint256");
        assert_eq!(test_log4.inputs[3].ty.to_string(), "uint256");
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
