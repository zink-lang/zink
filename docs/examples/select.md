# Select


```rust
//! if-else example.
#![no_std]

// for the panic handler.
#[cfg(not(test))]
extern crate zink;

/// Simple if-else condition
#[no_mangle]
pub extern "C" fn if_else(x: u64, y: u64) -> u64 {
    if x > y {
        x
    } else {
        y
    }
}

```

Code block selecting value with if-else will be compiled to instruction `select` in WASM

```wasm
(module
  (type (;0;) (func (param i64 i64) (result i64)))
  (func $if_else (type 0) (param i64 i64) (result i64)
    local.get 0
    local.get 1
    local.get 0
    local.get 1
    i64.gt_u
    select))
```

Since EVM bytecode doesn't have similar instruction, we have to implement it ourselves, the solution
is introduce a `select` function in the extra code section provided by zink compiler, jump to there
and jump back just like calling a real function.

```
60003560203560003560203510589190601c575b60005260206000f35b5060060156
```
