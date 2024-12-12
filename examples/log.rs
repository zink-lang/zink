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

    use zink::Asm;
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
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
