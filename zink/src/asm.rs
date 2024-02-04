//! Assembly trait implementation.

use crate::ffi;

/// Types implemented this trait are able to be pushed on stack.
pub trait Asm {
    /// Push self on the stack.
    fn push(&self);
}

impl Asm for i8 {
    fn push(&self) {
        unsafe {
            ffi::asm::push_i8(*self);
        }
    }
}
