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
                #[cfg(target_arch = "wasm32")]
                unsafe {
                    paste! { ffi::asm::[<push_ $ty>](self); }
                }
                #[cfg(not(target_arch = "wasm32"))]
                paste! { ffi::asm::[<push_ $ty>](self); }
            }

            #[cfg(not(target_family = "wasm"))]
            fn bytes32(&self) -> [u8; 32] {
                crate::to_bytes32(&self.to_le_bytes())
            }
        }
    };
    ($($ty:tt),+) => {
        $(impl_asm!($ty);)+
    };
}

impl_asm!(i8, u8, i16, u16, i32, u32, i64, u64);
