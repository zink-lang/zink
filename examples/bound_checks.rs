#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use zink::primitives::numeric::SafeNumeric;
#[allow(unused_imports)]
use zink::primitives::U256;

#[no_mangle]
pub fn addition(x: i32) -> i32 {
    x.safe_add(i32::MAX)
}

#[no_mangle]
pub fn subtraction(x: i32) -> i32 {
    x.safe_sub(i32::MIN)
}

#[no_mangle]
pub fn multiplication(x: i32) -> i32 {
    x.safe_mul(2)
}

#[no_mangle]
pub fn division(x: i32) -> i32 {
    x.safe_div(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    // i8 Tests
    #[test]
    #[should_panic(expected = "addition overflow")]
    fn test_add_overflow_i8() {
        i8::MAX.safe_add(1);
    }

    #[test]
    #[should_panic(expected = "subtraction overflow")]
    fn test_sub_overflow_i8() {
        0i8.safe_sub(i8::MIN);
    }

    #[test]
    #[should_panic(expected = "multiplication overflow")]
    fn test_mul_overflow_i8() {
        i8::MAX.safe_mul(2);
    }

    #[test]
    #[should_panic(expected = "division overflow")]
    fn test_div_overflow_i8() {
        i8::MIN.safe_div(-1);
    }

    // u8 Tests
    #[test]
    #[should_panic(expected = "addition overflow")]
    fn test_add_overflow_u8() {
        u8::MAX.safe_add(1);
    }

    #[test]
    #[should_panic(expected = "subtraction overflow")]
    fn test_sub_overflow_u8() {
        0u8.safe_sub(1);
    }

    #[test]
    #[should_panic(expected = "multiplication overflow")]
    fn test_mul_overflow_u8() {
        u8::MAX.safe_mul(2);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn test_div_by_zero_u8() {
        1u8.safe_div(0);
    }

    // i16 Tests
    #[test]
    #[should_panic(expected = "addition overflow")]
    fn test_add_overflow_i16() {
        i16::MAX.safe_add(1);
    }

    #[test]
    #[should_panic(expected = "subtraction overflow")]
    fn test_sub_overflow_i16() {
        0i16.safe_sub(i16::MIN);
    }

    #[test]
    #[should_panic(expected = "multiplication overflow")]
    fn test_mul_overflow_i16() {
        i16::MAX.safe_mul(2);
    }

    #[test]
    #[should_panic(expected = "division overflow")]
    fn test_div_overflow_i16() {
        i16::MIN.safe_div(-1);
    }

    // u16 Tests
    #[test]
    #[should_panic(expected = "addition overflow")]
    fn test_add_overflow_u16() {
        u16::MAX.safe_add(1);
    }

    #[test]
    #[should_panic(expected = "subtraction overflow")]
    fn test_sub_overflow_u16() {
        0u16.safe_sub(1);
    }

    #[test]
    #[should_panic(expected = "multiplication overflow")]
    fn test_mul_overflow_u16() {
        u16::MAX.safe_mul(2);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn test_div_by_zero_u16() {
        1u16.safe_div(0);
    }

    // i32 Tests (original)
    #[test]
    #[should_panic(expected = "addition overflow")]
    fn test_add_overflow_i32() {
        i32::MAX.safe_add(1);
    }

    #[test]
    #[should_panic(expected = "subtraction overflow")]
    fn test_sub_overflow_i32() {
        0i32.safe_sub(i32::MIN);
    }

    #[test]
    #[should_panic(expected = "multiplication overflow")]
    fn test_mul_overflow_i32() {
        i32::MAX.safe_mul(2);
    }

    #[test]
    #[should_panic(expected = "division overflow")]
    fn test_div_overflow_i32() {
        i32::MIN.safe_div(-1);
    }

    // u32 Tests (original)
    #[test]
    #[should_panic(expected = "addition overflow")]
    fn test_add_overflow_u32() {
        u32::MAX.safe_add(1);
    }

    #[test]
    #[should_panic(expected = "subtraction overflow")]
    fn test_sub_overflow_u32() {
        0u32.safe_sub(1);
    }

    #[test]
    #[should_panic(expected = "multiplication overflow")]
    fn test_mul_overflow_u32() {
        u32::MAX.safe_mul(2);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn test_div_by_zero_u32() {
        1u32.safe_div(0);
    }

    // i64 Tests (original)
    #[test]
    #[should_panic(expected = "addition overflow")]
    fn test_add_overflow_i64() {
        i64::MAX.safe_add(1);
    }

    #[test]
    #[should_panic(expected = "subtraction overflow")]
    fn test_sub_overflow_i64() {
        0i64.safe_sub(i64::MIN);
    }

    #[test]
    #[should_panic(expected = "multiplication overflow")]
    fn test_mul_overflow_i64() {
        i64::MAX.safe_mul(2);
    }

    #[test]
    #[should_panic(expected = "division overflow")]
    fn test_div_overflow_i64() {
        i64::MIN.safe_div(-1);
    }

    // u64 Tests (original)
    #[test]
    #[should_panic(expected = "addition overflow")]
    fn test_add_overflow_u64() {
        u64::MAX.safe_add(1);
    }

    #[test]
    #[should_panic(expected = "subtraction overflow")]
    fn test_sub_overflow_u64() {
        0u64.safe_sub(1);
    }

    #[test]
    #[should_panic(expected = "multiplication overflow")]
    fn test_mul_overflow_u64() {
        u64::MAX.safe_mul(2);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn test_div_by_zero_u64() {
        1u64.safe_div(0);
    }

    // U256 Tests (only run on WASM)
    #[cfg(target_arch = "wasm32")]
    #[test]
    #[should_panic(expected = "addition overflow")]
    fn test_add_overflow_u256() {
        U256::max().safe_add(U256::from(1u64));
    }

    #[cfg(target_arch = "wasm32")]
    #[test]
    #[should_panic(expected = "subtraction overflow")]
    fn test_sub_overflow_u256() {
        U256::empty().safe_sub(U256::from(1u64));
    }

    #[cfg(target_arch = "wasm32")]
    #[test]
    #[should_panic(expected = "multiplication overflow")]
    fn test_mul_overflow_u256() {
        U256::max().safe_mul(U256::from(2u64));
    }

    #[cfg(target_arch = "wasm32")]
    #[test]
    #[should_panic(expected = "division by zero")]
    fn test_div_by_zero_u256() {
        U256::from(1u64).safe_div(U256::empty());
    }

    // No-overflow Tests (covering all types)
    #[test]
    fn test_no_overflow() {
        // i8
        assert_eq!((-1i8).safe_add(i8::MAX), i8::MAX - 1);
        assert_eq!((-1i8).safe_sub(i8::MIN), -1 - i8::MIN);
        assert_eq!(2i8.safe_mul(2), 4);
        assert_eq!(2i8.safe_div(-1), -2);
        // u8
        assert_eq!(1u8.safe_add(2), 3);
        assert_eq!(3u8.safe_sub(1), 2);
        assert_eq!(2u8.safe_mul(2), 4);
        assert_eq!(4u8.safe_div(2), 2);
        // i16
        assert_eq!((-1i16).safe_add(i16::MAX), i16::MAX - 1);
        assert_eq!((-1i16).safe_sub(i16::MIN), -1 - i16::MIN);
        assert_eq!(2i16.safe_mul(2), 4);
        assert_eq!(2i16.safe_div(-1), -2);
        // u16
        assert_eq!(1u16.safe_add(2), 3);
        assert_eq!(3u16.safe_sub(1), 2);
        assert_eq!(2u16.safe_mul(2), 4);
        assert_eq!(4u16.safe_div(2), 2);
        // i32
        assert_eq!((-1i32).safe_add(i32::MAX), i32::MAX - 1);
        assert_eq!((-1i32).safe_sub(i32::MIN), -1 - i32::MIN);
        assert_eq!(2i32.safe_mul(2), 4);
        assert_eq!(2i32.safe_div(-1), -2);
        // u32
        assert_eq!(1u32.safe_add(2), 3);
        assert_eq!(3u32.safe_sub(1), 2);
        assert_eq!(2u32.safe_mul(2), 4);
        assert_eq!(4u32.safe_div(2), 2);
        // i64
        assert_eq!((-1i64).safe_add(i64::MAX), i64::MAX - 1);
        assert_eq!((-1i64).safe_sub(i64::MIN), -1 - i64::MIN);
        assert_eq!(2i64.safe_mul(2), 4);
        assert_eq!(2i64.safe_div(-1), -2);
        // u64
        assert_eq!(1u64.safe_add(2), 3);
        assert_eq!(3u64.safe_sub(1), 2);
        assert_eq!(2u64.safe_mul(2), 4);
        assert_eq!(4u64.safe_div(2), 2);
        #[cfg(target_arch = "wasm32")]
        {
            // U256
            assert_eq!(
                U256::from(1u64).safe_add(U256::from(2u64)),
                U256::from(3u64)
            );
            assert_eq!(
                U256::from(3u64).safe_sub(U256::from(1u64)),
                U256::from(2u64)
            );
            assert_eq!(
                U256::from(2u64).safe_mul(U256::from(2u64)),
                U256::from(4u64)
            );
            assert_eq!(
                U256::from(4u64).safe_div(U256::from(2u64)),
                U256::from(2u64)
            );
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
