use crate::ffi;

/// A trait for modular arithmetic operations on numeric types.
pub trait Numeric: Copy {
    fn addmod(self, other: Self, n: Self) -> Self;
    fn mulmod(self, other: Self, n: Self) -> Self;
}

macro_rules! impl_numeric {
    ($($t:ty, $addmod_fn:ident, $mulmod_fn:ident);* $(;)?) => {
        $(
            impl Numeric for $t {
                #[inline(always)]
                fn addmod(self, other: Self, n: Self) -> Self {
                    unsafe { ffi::asm::$addmod_fn(self, other, n) }
                }
                #[inline(always)]
                fn mulmod(self, other: Self, n: Self) -> Self {
                    unsafe { ffi::asm::$mulmod_fn(self, other, n) }
                }
            }
        )*
    };
}


impl_numeric! {
    i8, addmod_i8, mulmod_i8;
    u8, addmod_u8, mulmod_u8;
    i16, addmod_i16, mulmod_i16;
    u16, addmod_u16, mulmod_u16;
    i32, addmod_i32, mulmod_i32;
    u32, addmod_u32, mulmod_u32;
    i64, addmod_i64, mulmod_i64;
    u64, addmod_u64, mulmod_u64;
}
