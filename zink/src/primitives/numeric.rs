use crate::ffi;

/// A trait for modular arithmetic operations on numeric types.
pub trait Numeric: Copy {
    fn addmod(self, other: Self, n: Self) -> Self;
    fn mulmod(self, other: Self, n: Self) -> Self;
}

/// A trait for safe arithmetic operations with bound checks.
pub trait SafeNumeric: Copy + PartialOrd + Sized {
    const MAX: Self;
    const MIN: Self;

    fn safe_add(self, rhs: Self) -> Self;
    fn safe_sub(self, rhs: Self) -> Self;
    fn safe_mul(self, rhs: Self) -> Self;
    fn safe_div(self, rhs: Self) -> Self;
}

macro_rules! local_revert {
    ($msg:expr) => {
        unsafe { crate::ffi::asm::revert1($msg) }
    };
}

macro_rules! impl_numeric {
    ($($t:ty, $addmod_fn:ident, $mulmod_fn:ident);* $(;)?) => {
        $(
            impl Numeric for $t {
                #[inline(always)]
                fn addmod(self, other: Self, n: Self) -> Self {
                    unsafe { ffi::asm::$addmod_fn(n, other, self) }
                }
                #[inline(always)]
                fn mulmod(self, other: Self, n: Self) -> Self {
                    unsafe { ffi::asm::$mulmod_fn(n, other, self) }
                }
            }
        )*
    };
}

// Signed types (i32, i64)
macro_rules! impl_safe_numeric_signed {
    ($($t:ty);* $(;)?) => {
        $(
            impl SafeNumeric for $t {
                const MAX: Self = <$t>::MAX;
                const MIN: Self = <$t>::MIN;

                #[inline(always)]
                fn safe_add(self, rhs: Self) -> Self {
                    let result = self.wrapping_add(rhs);
                    if (self > 0 && rhs > 0 && result < self) ||
                       (self < 0 && rhs < 0 && result > self) {
                        local_revert!("addition overflow");
                    }
                    result
                }

                #[inline(always)]
                fn safe_sub(self, rhs: Self) -> Self {
                    let result = self.wrapping_sub(rhs);
                    if rhs < 0 && self > result {
                        local_revert!("subtraction overflow");
                    }
                    result
                }

                #[inline(always)]
                fn safe_mul(self, rhs: Self) -> Self {
                    let result = self.wrapping_mul(rhs);
                    if rhs != 0 && result / rhs != self {
                        local_revert!("multiplication overflow");
                    }
                    result
                }

                #[inline(always)]
                fn safe_div(self, rhs: Self) -> Self {
                    if rhs == 0 {
                        local_revert!("division by zero");
                    }
                    let result = self.wrapping_div(rhs);
                    if self == i32::MIN.into() && rhs == -1 {
                        local_revert!("division overflow");
                    }
                    result
                }
            }
        )*
    };
}

// Unsigned types (u32, u64)
macro_rules! impl_safe_numeric_unsigned {
    ($($t:ty);* $(;)?) => {
        $(
            impl SafeNumeric for $t {
                const MAX: Self = <$t>::MAX;
                const MIN: Self = <$t>::MIN;

                #[inline(always)]
                fn safe_add(self, rhs: Self) -> Self {
                    let result = self + rhs;
                    if result < self {
                        local_revert!("addition overflow");
                    }
                    result
                }

                #[inline(always)]
                fn safe_sub(self, rhs: Self) -> Self {
                    let result = self - rhs;
                    if result > self {
                        local_revert!("subtraction overflow");
                    }
                    result
                }

                #[inline(always)]
                fn safe_mul(self, rhs: Self) -> Self {
                    let result = self * rhs;
                    if rhs != 0 && result / rhs != self {
                        local_revert!("multiplication overflow");
                    }
                    result
                }

                #[inline(always)]
                fn safe_div(self, rhs: Self) -> Self {
                    if rhs == 0 {
                        local_revert!("division by zero");
                    }
                    let result = self / rhs;
                    result
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

impl_safe_numeric_signed! {
    i32;
    i64;
}

impl_safe_numeric_unsigned! {
    u32;
    u64;
}
