//! Assembly trait implementation.

use crate::ffi;
use paste::paste;

/// Types implemented this trait are able to be pushed on stack.
pub trait Asm: Copy {
    /// Push self on the stack.
    fn push(self);
}

macro_rules! impl_asm {
    ($ty:ident) => {
        impl Asm for $ty {
            fn push(self) {
                unsafe {
                    paste! { ffi::asm::[<push_ $ty>](self); }
                }
            }
        }
    };
    ($($ty:tt),+) => {
        $(impl_asm!($ty);)+
    };
}

impl_asm!(i8, u8, i16, u16, i32, u32, i64, u64);
