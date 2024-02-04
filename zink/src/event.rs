//! Public traits for the EVM interfaces
use crate::ffi;

/// Zink event interface
///
/// TODO: safety check for the length of the event name
pub trait Event {
    const NAME: &'static [u8];

    fn log0(&self) {
        unsafe {
            ffi::evm::log0(Self::NAME);
        }
    }

    fn log1(&self, topic: &'static [u8]) {
        unsafe {
            ffi::evm::log1(Self::NAME, topic);
        }
    }

    fn log2(&self, topic1: &'static [u8], topic2: &'static [u8]) {
        unsafe {
            ffi::evm::log2(Self::NAME, topic1, topic2);
        }
    }

    fn log3(&self, topic1: &'static [u8], topic2: &'static [u8], topic3: &'static [u8]) {
        unsafe {
            ffi::evm::log3(Self::NAME, topic1, topic2, topic3);
        }
    }

    fn log4(
        &self,
        topic1: &'static [u8],
        topic2: &'static [u8],
        topic3: &'static [u8],
        topic4: &'static [u8],
    ) {
        unsafe {
            ffi::evm::log4(Self::NAME, topic1, topic2, topic3, topic4);
        }
    }
}
