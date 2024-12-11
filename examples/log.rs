#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::{primitives::U256, Event};

#[derive(Event)]
pub enum MyEvent {
    /// Event with no topics
    Topic0,
    /// Event with one topic
    Topic1(U256),
    /// Event with two topics
    Topic2(U256, U256),
    /// Event with three topics
    Topic3(U256, U256, U256),
    /// Event with four topics
    Topic4(U256, U256, U256, U256),
}

pub mod event_tests {

    use super::*;

    /// Test log0
    #[zink::external]
    pub fn test_log0() {
        unsafe { zink::ffi::evm::log0(b"MyEvent") }
    }

    /// Test log1
    #[zink::external]
    pub fn test_log1(value: U256) {
        unsafe {
            let topic = value.bytes32();
            zink::ffi::evm::log1(topic, b"MyEvent")
        }
    }

    /// Test log2
    #[zink::external]
    pub fn test_log2(value1: U256, value2: U256) {
        unsafe {
            let topic1 = value1.bytes32();
            let topic2 = value2.bytes32();
            zink::ffi::evm::log2(topic1, topic2, b"MyEvent")
        }
    }

    /// Test log3
    #[zink::external]
    pub fn test_log3(value1: U256, value2: U256, value3: U256) {
        unsafe {
            let topic1 = value1.bytes32();
            let topic2 = value2.bytes32();
            let topic3 = value3.bytes32();
            zink::ffi::evm::log3(topic1, topic2, topic3, b"MyEvent")
        }
    }

    /// Test log4
    #[zink::external]
    pub fn test_log4(value1: U256, value2: U256, value3: U256, value4: U256) {
        unsafe {
            let topic1 = value1.bytes32();
            let topic2 = value2.bytes32();
            let topic3 = value3.bytes32();
            let topic4 = value4.bytes32();

            zink::ffi::evm::log4(topic1, topic2, topic3, topic4, b"MyEvent")
        }
    }

    /// Test multiple event logs in one transaction
    #[zink::external]
    pub fn test_multiple_logs(value1: U256, value2: U256, value3: U256, value4: U256) {
        test_log0();
        test_log1(value1);
        test_log2(value1, value2);
        test_log3(value1, value2, value3);
        test_log4(value1, value2, value3, value4);
    }
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
                    value1.bytes32()[0].to_vec(),
                    value2.bytes32()[0].to_vec(),
                ])
                .unwrap();
            assert!(!info.logs.is_empty());
            assert_eq!(info.logs[0].topics().to_vec(), value1.bytes32().to_vec());
            assert_eq!(
                info.logs[0].data.topics().to_vec(),
                value2.bytes32()[0].to_vec()
            );

            let info = contract
                .execute(&[
                    b"test_log3(uint256,uint256,uint256)".to_vec(),
                    value1.bytes32()[0].to_vec(),
                    value2.bytes32()[0].to_vec(),
                ])
                .unwrap();
            assert!(!info.logs.is_empty());
            assert_eq!(
                info.logs[0].data.data.to_vec(),
                value1.bytes32()[0].to_vec()
            );
            assert_eq!(
                info.logs[1].data.data.to_vec(),
                value2.bytes32()[0].to_vec()
            );
            assert_eq!(
                info.logs[2].data.data.to_vec(),
                value3.bytes32()[0].to_vec()
            );

            let info = contract
                .execute(&[
                    b"test_log4(uint256,uint256,uint256,uint256)".to_vec(),
                    value1.bytes32()[0].to_vec(),
                    value2.bytes32()[0].to_vec(),
                ])
                .unwrap();
            assert!(!info.logs.is_empty());
            assert_eq!(
                info.logs[0].data.data.to_vec(),
                value1.bytes32()[0].to_vec()
            );
            assert_eq!(
                info.logs[1].data.data.to_vec(),
                value2.bytes32()[0].to_vec()
            );
            assert_eq!(
                info.logs[2].data.data.to_vec(),
                value3.bytes32()[0].to_vec()
            );
            assert_eq!(
                info.logs[3].data.data.to_vec(),
                value4.bytes32()[0].to_vec()
            );

            let info = contract
                .execute(&[
                    b"test_multiple_logs(uint256,uint256,uint256,uint256)".to_vec(),
                    value1.bytes32()[0].to_vec(),
                    value2.bytes32()[0].to_vec(),
                    value3.bytes32()[0].to_vec(),
                    value4.bytes32()[0].to_vec(),
                ])
                .unwrap();
            assert!(!info.logs.is_empty());
            assert_eq!(
                info.logs[0].data.data.to_vec(),
                value1.bytes32()[0].to_vec()
            );
            assert_eq!(
                info.logs[1].data.data.to_vec(),
                value2.bytes32()[0].to_vec()
            );
            assert_eq!(
                info.logs[2].data.data.to_vec(),
                value3.bytes32()[0].to_vec()
            );
            assert_eq!(
                info.logs[3].data.data.to_vec(),
                value4.bytes32()[0].to_vec()
            );
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
