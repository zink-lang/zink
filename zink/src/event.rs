//! Event implementation

use crate::ffi;

/// Convert static value to pointer
pub trait AsPtr {
    fn ptr(&self) -> i32;
}

impl AsPtr for &'static str {
    fn ptr(&self) -> i32 {
        self.as_ptr() as i32
    }
}

impl AsPtr for &'static [u8] {
    fn ptr(&self) -> i32 {
        self.as_ptr() as i32
    }
}

impl AsPtr for i32 {
    fn ptr(&self) -> i32 {
        *self
    }
}

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

    fn log2(&self, topic1: impl AsPtr, topic2: impl AsPtr) {
        unsafe {
            ffi::evm::log2(Self::NAME.as_ptr() as i32, topic1.ptr(), topic2.ptr());
        }
    }

    fn log3(&self, topic1: impl AsPtr, topic2: impl AsPtr, topic3: impl AsPtr) {
        unsafe {
            ffi::evm::log3(
                Self::NAME.as_ptr() as i32,
                topic1.ptr(),
                topic2.ptr(),
                topic3.ptr(),
            );
        }
    }

    fn log4(&self, topic1: impl AsPtr, topic2: impl AsPtr, topic3: impl AsPtr, topic4: impl AsPtr) {
        unsafe {
            ffi::evm::log4(
                Self::NAME.as_ptr() as i32,
                topic1.ptr(),
                topic2.ptr(),
                topic3.ptr(),
                topic4.ptr(),
            );
        }
    }
}
