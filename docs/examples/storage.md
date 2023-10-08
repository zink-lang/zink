# Storage

```rust
// for the panic handler.
#[cfg(not(test))]
extern crate zink;

use zink::Storage;

/// It gets expanded to 'Counter' struct
/// that implements zink::Storage trait
/// (::set and ::get)
///
/// Storage key is taken based on macro order
/// (e.g this macro is first and only in this project,
/// so it will take 0x0 contract storage key)
#[zink::storage]
pub type Counter = i32;

/// Set value to storage and get it
#[no_mangle]
pub unsafe extern "C" fn set_and_get(value: i64) -> i64 {
    Counter::set(value);
    Counter::get()
}
```

Simple storage IO for numbers.

```wasm
(module
  (type (;0;) (func (param i64) (result i64)))
  (type (;1;) (func (param i64 i64)))
  (import "zink" "sload" (func (;0;) (type 0)))
  (import "zink" "sstore" (func (;1;) (type 1)))
  (func (type 0) (param i64) (result i64)
        i64.const 0
        local.get 0
        call 1
        i64.const 0
        call 0))
```

Set and get number parameter with storage here.

```
60006000355891601b565b600058906021565b60005260206000f35b55600501565b549060050156
```
