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
    use std::{fs, path::Path};
    use zint::{EVM, Bytes32};

    #[test]
    fn test_addition() {
        // Assumes `elko build` has run and produced target/zink/addition.bin
        let bytecode = fs::read("target/zink/addition.bin").expect("Failed to read addition.bin");
        let inputs = vec![
            2u64.to_le_bytes().to_vec(),
            3u64.to_le_bytes().to_vec(),
        ];
        let calldata = inputs.iter().fold(Vec::new(), |mut acc, input| {
            acc.extend_from_slice(&input.to_bytes32());
            acc
        });
        let info = EVM::interp(&bytecode, &calldata).expect("Failed to execute addition");
        let result = info.ret;
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
