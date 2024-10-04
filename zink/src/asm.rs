//! Assembly trait implementation.

use crate::ffi;
use paste::paste;

/// Types implemented this trait are able to be pushed on stack.
pub trait Asm: Copy {
    /// Push self on the stack.
    fn push(self);

    #[cfg(not(target_family = "wasm"))]
    fn bytes32(&self) -> [u8; 32];
}

macro_rules! impl_asm {
    ($ty:ident) => {
        impl Asm for $ty {
            fn push(self) {
                unsafe {
                    paste! { ffi::asm::[<push_ $ty>](self); }
                }
            }

            #[cfg(not(target_family = "wasm"))]
            fn bytes32(&self) -> [u8; 32] {
                let mut output = [0; 32];
                let bytes = self.to_le_bytes();
                output[(32 - bytes.len())..].copy_from_slice(&bytes);
                output
            }
        }
    };
    ($($ty:tt),+) => {
        $(impl_asm!($ty);)+
    };
}

impl_asm!(i8, u8, i16, u16, i32, u32, i64, u64);
