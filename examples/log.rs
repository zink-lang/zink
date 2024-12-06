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
    pub fn test_log0() -> Result<(), ()> {
        unsafe { zink::ffi::evm::log0(b"MyEvent") }

        Ok(())
    }

    /// Test log1
    #[zink::external]
    pub fn test_log1(value: U256) -> Result<(), ()> {
        unsafe {
            let topic = value.bytes32();
            zink::ffi::evm::log1(b"MyEvent", topic)
        }

        Ok(())
    }

    /// Test log2
    #[zink::external]
    pub fn test_log2(value1: U256, value2: U256) -> Result<(), ()> {
        unsafe {
            let topic1 = value1.bytes32();
            let topic2 = value2.bytes32();
            zink::ffi::evm::log2(b"MyEvent", topic1, topic2)
        }
        Ok(())
    }

    /// Test log3
    #[zink::external]
    pub fn test_log3(value1: U256, value2: U256, value3: U256) -> Result<(), ()> {
        unsafe {
            let topic1 = value1.bytes32();
            let topic2 = value2.bytes32();
            let topic3 = value3.bytes32();
            zink::ffi::evm::log3(b"MyEvent", topic1, topic2, topic3)
        }
        Ok(())
    }

    /// Test log4
    #[zink::external]
    pub fn test_log4(value1: U256, value2: U256, value3: U256, value4: U256) -> Result<(), ()> {
        unsafe {
            let topic1 = value1.bytes32();
            let topic2 = value2.bytes32();
            let topic3 = value3.bytes32();
            let topic4 = value4.bytes32();

            zink::ffi::evm::log4(b"MyEvent", topic1, topic2, topic3, topic4)
        }
        Ok(())
    }

    /// Test multiple event logs in one transaction
    #[zink::external]
    pub fn test_multiple_logs(
        value1: U256,
        value2: U256,
        value3: U256,
        value4: U256,
    ) -> Result<(), ()> {
        test_log0().unwrap();
        test_log1(value1).unwrap();
        test_log2(value1, value2).unwrap();
        test_log3(value1, value2, value3).unwrap();
        test_log4(value1, value2, value3, value4).unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use zint::{Bytes32, Contract};

    #[test]
    fn test_events() {
        let mut contract = Contract::search("log").unwrap().compile().unwrap();

        let value1 = U256::from(U256::empty());
        let value2 = U256::from(U256::empty());
        let value3 = U256::from(U256::empty());
        let value4 = U256::from(U256::empty());

        {
            let info = contract
                .execute(&[b"test_log0(U256,U256)".to_vec()])
                .unwrap();
            assert!(!info.logs.is_empty());

            let info = contract
                .execute(&[b"test_log1(U256)".to_vec(), value1.bytes32().0.to_vec()])
                .unwrap();
            assert!(!info.logs.is_empty());
            assert_eq!(info.logs[0].data.data.to_vec(), value1.bytes32().0.to_vec());

            let info = contract
                .execute(&[
                    b"test_log2(U256,U256)".to_vec(),
                    value1.bytes32().0.to_vec(),
                    value2.bytes32().0.to_vec(),
                ])
                .unwrap();
            assert!(!info.logs.is_empty());
            assert_eq!(info.logs[0].data.data.to_vec(), value1.bytes32().0.to_vec());
            assert_eq!(info.logs[1].data.data.to_vec(), value2.bytes32().0.to_vec());

            let info = contract
                .execute(&[
                    b"test_log3(U256,U256,U256)".to_vec(),
                    value1.bytes32().0.to_vec(),
                    value2.bytes32().0.to_vec(),
                ])
                .unwrap();
            assert!(!info.logs.is_empty());
            assert_eq!(info.logs[0].data.data.to_vec(), value1.bytes32().0.to_vec());
            assert_eq!(info.logs[1].data.data.to_vec(), value2.bytes32().0.to_vec());
            assert_eq!(info.logs[2].data.data.to_vec(), value3.bytes32().0.to_vec());

            let info = contract
                .execute(&[
                    b"test_log4(U256,U256,U256,U256)".to_vec(),
                    value1.bytes32().0.to_vec(),
                    value2.bytes32().0.to_vec(),
                ])
                .unwrap();
            assert!(!info.logs.is_empty());
            assert_eq!(info.logs[0].data.data.to_vec(), value1.bytes32().0.to_vec());
            assert_eq!(info.logs[1].data.data.to_vec(), value2.bytes32().0.to_vec());
            assert_eq!(info.logs[2].data.data.to_vec(), value3.bytes32().0.to_vec());
            assert_eq!(info.logs[3].data.data.to_vec(), value4.bytes32().0.to_vec());

            let info = contract
                .execute(&[
                    b"test_multiple_logs(U256,U256,U256,U256)".to_vec(),
                    value1.bytes32().0.to_vec(),
                    value2.bytes32().0.to_vec(),
                    value3.bytes32().0.to_vec(),
                    value4.bytes32().0.to_vec(),
                ])
                .unwrap();
            assert!(!info.logs.is_empty());
            assert_eq!(info.logs[0].data.data.to_vec(), value1.bytes32().0.to_vec());
            assert_eq!(info.logs[1].data.data.to_vec(), value2.bytes32().0.to_vec());
            assert_eq!(info.logs[2].data.data.to_vec(), value3.bytes32().0.to_vec());
            assert_eq!(info.logs[3].data.data.to_vec(), value4.bytes32().0.to_vec());
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
