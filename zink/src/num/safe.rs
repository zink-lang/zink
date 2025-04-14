//! Numeric primitives

use crate::{asm, primitives::U256};

/// A trait for safe arithmetic operations with bound checks.
pub trait SafeNumeric: Copy + PartialOrd + Sized {
    fn max() -> Self;
    fn min() -> Self;

    fn safe_add(self, rhs: Self) -> Self;
    fn safe_sub(self, rhs: Self) -> Self;
    fn safe_mul(self, rhs: Self) -> Self;
    fn safe_div(self, rhs: Self) -> Self;
}

macro_rules! local_revert {
    ($msg:expr) => {
        unsafe {
            crate::asm::ext::revert1($msg);
        }
    };
}

// Signed types (i8, i16, i32, i64)
macro_rules! impl_safe_numeric_signed {
    ($($t:ty);* $(;)?) => {
        $(
            impl SafeNumeric for $t {
                #[inline(always)]
                fn max() -> Self { <$t>::MAX }
                #[inline(always)]
                fn min() -> Self { <$t>::MIN }

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
                    if self == <Self as SafeNumeric>::min() && rhs == -1 {
                        local_revert!("division overflow");
                    }
                    result
                }
            }
        )*
    };
}

// Unsigned types (u8, u16, u32, u64)
macro_rules! impl_safe_numeric_unsigned {
    ($($t:ty);* $(;)?) => {
        $(
            impl SafeNumeric for $t {
                #[inline(always)]
                fn max() -> Self { <$t>::MAX }
                #[inline(always)]
                fn min() -> Self { <$t>::MIN }

                #[inline(always)]
                fn safe_add(self, rhs: Self) -> Self {
                    let result = self.wrapping_add(rhs);
                    if result < self {
                        local_revert!("addition overflow");
                    }
                    result
                }

                #[inline(always)]
                fn safe_sub(self, rhs: Self) -> Self {
                    let result = self.wrapping_sub(rhs);
                    if result > self {
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
                    self / rhs
                }
            }
        )*
    };
}

// U256 special case
impl SafeNumeric for U256 {
    #[inline(always)]
    fn max() -> Self {
        unsafe { asm::ext::u256_max() }
    }
    #[inline(always)]
    fn min() -> Self {
        U256::empty()
    }

    #[inline(always)]
    fn safe_add(self, rhs: Self) -> Self {
        let result = unsafe { asm::ext::u256_add(self, rhs) };
        if result < self {
            local_revert!("addition overflow");
        }
        result
    }

    #[inline(always)]
    fn safe_sub(self, rhs: Self) -> Self {
        let result = unsafe { asm::ext::u256_sub(self, rhs) };
        if result > self {
            local_revert!("subtraction overflow");
        }
        result
    }

    #[inline(always)]
    fn safe_mul(self, rhs: Self) -> Self {
        let max = Self::max();
        let result = unsafe { asm::ext::u256_mulmod(self, rhs, max) };
        // Check if result exceeds max when rhs > 1
        if rhs > Self::min() && result > self && result > rhs && result > max - self {
            local_revert!("multiplication overflow");
        }
        result
    }

    #[inline(always)]
    fn safe_div(self, rhs: Self) -> Self {
        if rhs == Self::min() {
            local_revert!("division by zero");
        }
        unsafe { asm::ext::u256_div(self, rhs) }
    }
}

impl_safe_numeric_signed! {
    i8;
    i16;
    i32;
    i64;
}

impl_safe_numeric_unsigned! {
    u8;
    u16;
    u32;
    u64;
}
