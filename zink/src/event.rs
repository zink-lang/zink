
use crate::{ffi, primitives::Bytes32};

/// Zink event interface
pub trait Event {
    const NAME: &'static [u8];

    fn log0(&self) {
        unsafe {
            ffi::evm::log0(Self::NAME);
        }
    }

    fn log1(&self, topic: impl Into<Bytes32>) {
        unsafe {
            ffi::evm::log1(Self::NAME, topic.into())
        }
    }

    fn log2(&self, topic1: impl Into<Bytes32>, topic2: impl Into<Bytes32>) {
        unsafe {
            ffi::evm::log2(Self::NAME, topic1.into(), topic2.into())
        }
    }

    fn log3(
        &self,
        topic1: impl Into<Bytes32>,
        topic2: impl Into<Bytes32>,
        topic3: impl Into<Bytes32>
    ) {
        unsafe {
            ffi::evm::log3(Self::NAME, topic1.into(), topic2.into(), topic3.into())
        }
    }

    fn log4(
        &self,
        topic1: impl Into<Bytes32>,
        topic2: impl Into<Bytes32>,
        topic3: impl Into<Bytes32>,
        topic4: impl Into<Bytes32>
    ) {
        unsafe {
            ffi::evm::log4(
                Self::NAME,
                topic1.into(),
                topic2.into(),
                topic3.into(),
                topic4.into()
            )
        }
    }
}

