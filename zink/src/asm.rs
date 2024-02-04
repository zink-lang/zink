//! Assembly trait implementation.

use crate::ffi;
use paste::paste;

/// Types implemented this trait are able to be pushed on stack.
pub trait Asm {
    /// Push self on the stack.
    fn push(&self);
}

macro_rules! impl_asm {
    ($ty:ident) => {
        impl Asm for $ty {
            fn push(&self) {
                unsafe {
                    paste! { ffi::asm::[<push_ $ty>](*self); }
                }
            }
        }
    };
    ($len:expr) => {
        impl Asm for [u8; $len] {
            fn push(&self) {
                unsafe {
                    paste! { ffi::evm::[<push $len>](self.as_ptr() as i32); }
                }
            }
        }
    };
    ($($ty:tt),+) => {
        $(impl_asm!($ty);)+
    };
}

impl_asm!(
    i8, u8, i16, u16, i32, u32, i64, u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
