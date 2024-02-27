# Solidity ABIA

An straightforward solidity ABI implementation for zink.

This library only contains a part of the solidity ABI implementation, 
if you are looking for a complete implementation of solidity ABI 
in rust, see [alloy-core][alloy-core].

## Static Types

Only rust primitive types are supported in this static type port,

| rust   | solidity  |
|--------|-----------|
| `i8`   | `int8`    |
| `u8`   | `uint8`   |
| `i16`  | `int16`   |
| `u16`  | `uint16`  |
| `i32`  | `int32`   |
| `u32`  | `uint32`  |
| `i64`  | `int64`   |
| `u64`  | `uint64`  |
| `i128` | `int128`  |
| `u128` | `uint128` |
| `bool` | `bool`    |


## Dynamic Types

The implementation of dynamic arguments follows [use-of-dynamic-types][dyn-types],
same as the static types, only ports the rust types:

| rust       | solidity  |
|------------|-----------|
| `Vec<u8>`  | `bytes`   |
| `[u8; 20]` | `address` |
| `String`   | `string`  |

More complex types are currently not supported.


## LICENSE

GPL-3.0


[alloy-core]: https://github.com/alloy-rs/core
[dyn-types]: https://docs.soliditylang.org/en/latest/abi-spec.html#use-of-dynamic-types
[zink]: https://github.com/zink-lang/zink
