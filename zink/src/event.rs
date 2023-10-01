//! Event implementation

use crate::ffi;

/// Zink event interface
pub trait Event {
    const NAME: &'static [u8];

    fn log0(&self) {
        unsafe {
            // TODO: safety check for the length of the event name
            ffi::evm::log0(Self::NAME.as_ptr() as i32);
        }
    }

    fn log1(&self) {
        unsafe {
            // TODO: safety check for the length of the event name
            ffi::evm::log1(Self::NAME.as_ptr() as i32);
        }
    }

    fn log2(&self) {
        unsafe {
            // TODO: safety check for the length of the event name
            ffi::evm::log2(Self::NAME.as_ptr() as i32);
        }
    }

    fn log3(&self) {
        unsafe {
            // TODO: safety check for the length of the event name
            ffi::evm::log3(Self::NAME.as_ptr() as i32);
        }
    }

    fn log4(&self) {
        unsafe {
            // TODO: safety check for the length of the event name
            ffi::evm::log4(Self::NAME.as_ptr() as i32);
        }
    }
}
