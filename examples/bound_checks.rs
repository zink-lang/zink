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
        addition(1); // 1 + i32::MAX
    }

    #[test]
    #[should_panic(expected = "subtraction overflow")]
    fn test_sub_overflow() {
        subtraction(i32::MAX); // i32::MAX - i32::MIN
    }

    #[test]
    #[should_panic(expected = "multiplication overflow")]
    fn test_mul_overflow() {
        multiplication(i32::MAX); // i32::MAX * 2
    }

    #[test]
    #[should_panic(expected = "division overflow")]
    fn test_div_overflow() {
        division(i32::MIN); // i32::MIN / -1
    }

    #[test]
    fn test_no_overflow() {
        assert_eq!(addition(-1), i32::MAX - 1);
        assert_eq!(subtraction(0), 0i32.wrapping_sub(i32::MIN)); // used wrapping_sub to avoid overflow in test
        assert_eq!(multiplication(2), 4);
        assert_eq!(division(2), -2);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
