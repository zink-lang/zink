//! Extended traits for primitives

use crate::asm;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
pub use safe::SafeNumeric;

mod safe;

/// A trait for modular arithmetic operations on numeric types.
pub trait Numeric:
    Add
    + AddAssign
    + Mul
    + MulAssign
    + Sub
    + SubAssign
    + Div
    + DivAssign
    + Sized
    + PartialEq
    + PartialOrd
{
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
                    unsafe { asm::ext::[<addmod_ $t>](n, other, self) }
                }

                #[inline(always)]
                fn mulmod(self, other: Self, n: Self) -> Self {
                    unsafe { asm::ext::[<mulmod_ $t>](n, other, self) }
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
