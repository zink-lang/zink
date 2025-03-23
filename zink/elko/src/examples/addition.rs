use crate::examples::Example;
/// Default "addition" example.
pub const ADDITION: Example = Example {
    lib_rs: r#"
//! ${name}
#![no_std]
#[cfg(not(test))]
extern crate zink;

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

    #[test]
    fn test_addition() {
        // Build the contract first (assumes `elko build` has run)
        let contract = Contract::search("addition").expect("Failed to find addition contract");
        let result = contract
            .call(&[2u64.to_le_bytes().to_vec(), 3u64.to_le_bytes().to_vec()])
            .expect("Failed to call addition");
        let expected = 5u64.to_le_bytes().to_vec();
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
