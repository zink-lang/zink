use crate::{ffi, primitives::Bytes32};

/// Zink event interface
pub trait Event {
    const NAME: &'static [u8];

    fn log0(&self) {
        #[cfg(target_arch = "wasm32")]
        unsafe {
            ffi::evm::log0(Self::NAME);
        }
        #[cfg(not(target_arch = "wasm32"))]
        ffi::evm::log0(Self::NAME);
    }

    fn log1(&self, topic: impl Into<Bytes32>) {
        #[cfg(target_arch = "wasm32")]
        unsafe {
            ffi::evm::log1(topic.into(), Self::NAME)
        }
        #[cfg(not(target_arch = "wasm32"))]
        ffi::evm::log1(topic.into(), Self::NAME)
    }

    fn log2(&self, topic1: impl Into<Bytes32>, topic2: impl Into<Bytes32>) {
        #[cfg(target_arch = "wasm32")]
        unsafe {
            ffi::evm::log2(topic1.into(), topic2.into(), Self::NAME)
        }
        #[cfg(not(target_arch = "wasm32"))]
        ffi::evm::log2(topic1.into(), topic2.into(), Self::NAME)
    }

    fn log3(
        &self,
        topic1: impl Into<Bytes32>,
        topic2: impl Into<Bytes32>,
        topic3: impl Into<Bytes32>,
    ) {
        #[cfg(target_arch = "wasm32")]
        unsafe {
            ffi::evm::log3(topic1.into(), topic2.into(), topic3.into(), Self::NAME)
        }
        #[cfg(not(target_arch = "wasm32"))]
        ffi::evm::log3(topic1.into(), topic2.into(), topic3.into(), Self::NAME)
    }

    fn log4(
        &self,
        topic1: impl Into<Bytes32>,
        topic2: impl Into<Bytes32>,
        topic3: impl Into<Bytes32>,
        topic4: impl Into<Bytes32>,
    ) {
        #[cfg(target_arch = "wasm32")]
        unsafe {
            ffi::evm::log4(
                topic1.into(),
                topic2.into(),
                topic3.into(),
                topic4.into(),
                Self::NAME,
            )
        }
        #[cfg(not(target_arch = "wasm32"))]
        ffi::evm::log4(
            topic1.into(),
            topic2.into(),
            topic3.into(),
            topic4.into(),
            Self::NAME,
        )
    }
}
