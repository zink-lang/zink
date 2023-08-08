# AddTwo

```rust
//! Addition example.
#![no_std]

// for the panic handler.
#[cfg(not(test))]
extern crate zink;

/// Adds two numbers together.
#[no_mangle]
pub extern "C" fn addition(x: u64, y: u64) -> u64 {
    x + y
}
```

A basic addition program in zink

```wasm
(module
    (func (param i32) (param i32) (result i32)
    (local.get 0)
    (local.get 1)
    (i32.add)
    )
)
```

Requires: 
- Get params from locals
- Process basic operand
- Return data from the result type

```text
6000356020350160005260206000f3
```
