# Log

```rust
//! Addition example.
#![no_std]

// for the panic handler.
#[cfg(not(test))]
extern crate zink;

use zink::Event;

/// A `Ping` event.
///
/// TODO: generate this with proc-macro.
struct Ping;

/// TODO: generate this with proc-macro.
impl Event for Ping {
    const NAME: &'static [u8] = b"Ping";
}

#[no_mangle]
pub extern "C" fn log1() {
    Ping.log1(b"pong");
}
```

The log API of zink is derived by the trait `Event` which provides methods
`log0`, `log1`, `log2`, `log3`, `log4`. We current only supports static
bytes in this API.

```wasm
(module
  (type (;0;) (func))
  (type (;1;) (func (param i32 i32 i32 i32)))
  (import "evm" "log1" (func (;1;) (type 1)))
  (import "env" "memory" (memory (;0;) 17))
  (func (;1;) (type 0)
    i32.const 1048576
    i32.const 4
    i32.const 1048580
    i32.const 4
    call 0)
  (export "log1" (func 1))
  (data (;0;) (i32.const 1048576) "Pingpong"))
```

The static byte array will be compiled to the data section of wasm, `zinkc`
gets it from the data section then process it to the logging interfaces.

```
63706f6e676350696e6760005260206000a15f5ff3
```
