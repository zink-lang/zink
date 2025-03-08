#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use zink::primitives::numeric::SafeNumeric;

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

    #[test]
    #[should_panic(expected = "addition overflow")]
    fn test_add_overflow() {
        i32::MAX.safe_add(1);
    }

    #[test]
    #[should_panic(expected = "subtraction overflow")]
    fn test_sub_overflow() {
        0.safe_sub(i32::MIN);
    }

    #[test]
    #[should_panic(expected = "multiplication overflow")]
    fn test_mul_overflow() {
        i32::MAX.safe_mul(2);
    }

    #[test]
    #[should_panic(expected = "division overflow")]
    fn test_div_overflow() {
        i32::MIN.safe_div(-1);
    }

    #[test]
    fn test_no_overflow() {
        assert_eq!((-1).safe_add(i32::MAX), i32::MAX - 1);
        assert_eq!((-1).safe_sub(i32::MIN), -1 - i32::MIN);
        assert_eq!(2.safe_mul(2), 4);
        assert_eq!(2.safe_div(-1), -2);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
