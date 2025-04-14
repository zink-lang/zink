//! Zink storage implementation.

use crate::asm;
pub use {
    dkmapping::{DoubleKeyMapping, DoubleKeyTransientMapping},
    mapping::{Mapping, TransientMapping},
    value::{Storage, TransientStorage},
};

pub mod dkmapping;
pub mod mapping;
mod value;

/// Trait for the value used in assembly code
pub trait Value {
    /// Load from storage
    fn sload() -> Self;

    /// Load from transient storage
    fn tload() -> Self;

    /// Push self on the stack.
    fn push(self);

    /// Convert to bytes32
    #[cfg(not(target_family = "wasm"))]
    fn bytes32(&self) -> [u8; 32];
}

macro_rules! impl_value {
    ($($ty:ident),+) => {
        $(
            paste::paste! {
                impl Value for $ty {
                    fn sload() -> Self {
                        unsafe { asm::ext::[<sload_ $ty>]() }
                    }

                    fn tload() -> Self {
                        unsafe { asm::ext::[<tload_ $ty>]() }
                    }

                    fn push(self) {
                        unsafe { asm::ext::[<push_ $ty>](self); }
                    }

                    #[cfg(not(target_family = "wasm"))]
                    fn bytes32(&self) -> [u8; 32] {
                            crate::to_bytes32(&self.to_le_bytes())
                    }
               }
            }
        )+
    };
}

impl_value!(i8, u8, i16, u16, i32, u32, i64, u64);
