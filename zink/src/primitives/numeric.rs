use crate::{ffi, primitives::U256};

/// A trait for modular arithmetic operations on numeric types.
pub trait Numeric: Copy {
    fn addmod(self, other: Self, n: Self) -> Self;
    fn mulmod(self, other: Self, n: Self) -> Self;
}

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
        #[cfg(target_arch = "wasm32")]
        unsafe {
            crate::ffi::asm::revert1($msg)
        }
        #[cfg(not(target_arch = "wasm32"))]
        crate::ffi::asm::asm::revert1($msg)
    };
}

macro_rules! impl_numeric {
    ($($t:ty, $addmod_fn:ident, $mulmod_fn:ident);* $(;)?) => {
        $(
            impl Numeric for $t {
                #[inline(always)]
                fn addmod(self, other: Self, n: Self) -> Self {
                    #[cfg(target_arch = "wasm32")]
                    unsafe { ffi::asm::$addmod_fn(n, other, self) }
                    #[cfg(not(target_arch = "wasm32"))]
                    ffi::asm::asm::$addmod_fn(n, other, self)
                }
                #[inline(always)]
                fn mulmod(self, other: Self, n: Self) -> Self {
                    #[cfg(target_arch = "wasm32")]
                    unsafe { ffi::asm::$mulmod_fn(n, other, self) }
                    #[cfg(not(target_arch = "wasm32"))]
                    ffi::asm::asm::$mulmod_fn(n, other, self)
                }
            }
        )*
    };
    // Special case for U256
    (U256, $addmod_fn:ident, $mulmod_fn:ident) => {
        impl Numeric for U256 {
            #[inline(always)]
            fn addmod(self, other: Self, n: Self) -> Self {
                unsafe { ffi::$addmod_fn(n, other, self) }
            }
            #[inline(always)]
            fn mulmod(self, other: Self, n: Self) -> Self {
                unsafe { ffi::$mulmod_fn(n, other, self) }
            }
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
        unsafe { ffi::u256_max() }
    }
    #[inline(always)]
    fn min() -> Self {
        U256::empty()
    }

    #[inline(always)]
    fn safe_add(self, rhs: Self) -> Self {
        let result = unsafe { ffi::u256_add(self, rhs) };
        if result < self {
            local_revert!("addition overflow");
        }
        result
    }

    #[inline(always)]
    fn safe_sub(self, rhs: Self) -> Self {
        let result = unsafe { ffi::u256_sub(self, rhs) };
        if result > self {
            local_revert!("subtraction overflow");
        }
        result
    }

    #[inline(always)]
    fn safe_mul(self, rhs: Self) -> Self {
        let max = Self::max();
        let result = unsafe { ffi::u256_mulmod(self, rhs, max) };
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
        unsafe { ffi::u256_div(self, rhs) }
    }
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
