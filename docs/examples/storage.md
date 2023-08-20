# Storage

```rust
use zink::storage::{sload, sstore};

/// TODO: generate this storage interface with proc macro.
struct Counter;

// The number `0` in this struct is for the storage key,
// it will be convreted to `0x000..0000`.
impl Counter {
    fn get() -> i64 {
        unsafe { sload(0) }
    }

    fn set(value: i64) {
        unsafe { sstore(0, value) }
    }
}

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
