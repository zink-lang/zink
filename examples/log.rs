#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::{primitives::U256, Asm, Event};

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
            let topic = value.to_bytes32();
            zink::ffi::evm::log1(b"MyEvent", topic)
        }
    }

    /// Test log2 
    #[zink::external]
    pub fn test_log2(value1: U256, value2: U256) {
        unsafe {
            let topic1 = value1.to_bytes32();
            let topic2 = value2.to_bytes32();
            zink::ffi::evm::log2(b"MyEvent", topic1, topic2)
        }
    }

    /// Test log3 
    #[zink::external]
    pub fn test_log3(value1: U256, value2: U256, value3: U256) {
        unsafe {
            let topic1 = value1.to_bytes32();
            let topic2 = value2.to_bytes32();
            let topic3 = value3.to_bytes32();
            zink::ffi::evm::log3(b"MyEvent", topic1, topic2, topic3)
        }
    }

    /// Test log4 
    #[zink::external]
    pub fn test_log4(value1: U256, value2: U256, value3: U256, value4: U256) {
        unsafe {
            let topic1 = value1.to_bytes32();
            let topic2 = value2.to_bytes32();
            let topic3 = value3.to_bytes32();
            let topic4 = value4.to_bytes32();
            zink::ffi::evm::log4(b"MyEvent", topic1, topic2, topic3, topic4)
        }
    }

    /// Test multiple event logs in one transaction
    #[zink::external]
    pub fn test_multiple_logs(
        value1: U256,
        value2: U256,
        value3: U256,
        value4: U256,
    ) -> Result<(), ()> {
        test_log0();
        test_log1(value1);
        test_log2(value1, value2);
        test_log3(value1, value2, value3);
        test_log4(value1, value2, value3, value4);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_events() {
        let value1 = U256::from(U256::empty());
        let value2 = U256::from(U256::empty());
        let value3 = U256::from(U256::empty());
        let value4 = U256::from(U256::empty());

        // Test each log function
        event_tests::test_log0();
        event_tests::test_log1(value1);
        event_tests::test_log2(value1, value2);
        event_tests::test_log3(value1, value2, value3);
        event_tests::test_log4(value1, value2, value3, value4);

        // Test multiple logs
        event_tests::test_multiple_logs(value1, value2, value3, value4).unwrap();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
