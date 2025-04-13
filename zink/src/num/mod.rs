//! Extended traits for primitives

use crate::ffi;

/// A trait for modular arithmetic operations on numeric types.
pub trait Numeric: Copy {
    /// Add modulo
    fn addmod(self, other: Self, n: Self) -> Self;

    /// Multiply modulo
    fn mulmod(self, other: Self, n: Self) -> Self;
}

macro_rules! impl_numeric {
    ($($t:ty),+) => {
        paste::paste!{
        $(
            impl Numeric for $t {
                #[inline(always)]
                fn addmod(self, other: Self, n: Self) -> Self {
                    unsafe { ffi::asm::[<addmod_ $t>](n, other, self) }
                }

                #[inline(always)]
                fn mulmod(self, other: Self, n: Self) -> Self {
                    unsafe { ffi::asm::[<mulmod_ $t>](n, other, self) }
                }
            }
        )*
    }
    };
}

impl_numeric! {
    i8,
    u8,
    i16,
    u16,
    i32,
    u32,
    i64,
    u64
}
