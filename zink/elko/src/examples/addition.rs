use crate::examples::Example;
/// Default "addition" example.
pub const ADDITION: Example = Example {
    lib_rs: r#"
//! ${name}
#![no_std]
#[cfg(not(test))]
extern crate zink;

#[cfg(all(test, not(target_arch = "wasm32")))]
extern crate alloc;

/// Adds two numbers together.
#[no_mangle]
pub extern "C" fn addition(x: u64, y: u64) -> u64 {
    x + y
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[cfg(test)]
mod tests {
    use zint::Contract;
    use alloc::vec;

    #[test]
    fn test_addition() {
        // Assumes `elko build` has run and produced target/zink/addition.wasm
        let contract = Contract::search("addition").expect("Failed to find addition contract");
        let mut contract = contract
            .pure()
            .compile()
            .expect("Failed to compile contract");
        let inputs = vec![
            2u64.to_le_bytes().to_vec(),
            3u64.to_le_bytes().to_vec(),
        ];
        let info = contract.execute(&inputs).expect("Failed to execute addition");
        let result = info.ret;
        let mut expected = [0u8; 32];
        let expected_bytes = 5u64.to_le_bytes();
        expected[24..32].copy_from_slice(&expected_bytes);
        assert_eq!(result, expected, "addition(2, 3) should return 5");
    }
}
"#,
    readme: r#"
# ${name}

> An EVM contract written in Rust with [The Zink Project][zink].

## Getting Started

```
cargo install zinkup
elko build
ls target/zink/${name}.bin
```

[zink]: https://github.com/zink-lang/zink
"#,
};
