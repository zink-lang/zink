# Security

## Mapping of Instructions

### Type Conversions

WASM have `i32`, `i64`, `f32`, `f64` as number types while EVM bytecode
only supports arithmetic operations for 256-bits integers.

> TODO: Add more risk conditions.

### Stack Operations

> TBA

### Memory Operations

The memory related operations in WASM are dangerous for Zink's implementation.

WASM is using 32-bits offsets from the MVP spec while EVM is using 256-bits offsets,
so it may cause memory overwrite problems.

The instructions need to be checked:

- `i32.store`
- `i64.store`
- `f32.store`
- `f64.store`
- `i32.store8`
- `i64.store8`
- `i32.store16`
- `i64.store16`
- `memory.size`
- `memory.grow`

TODO: check if it is possible to manage this issue with handling `memory.size` and `memory.grow`
in a proper way.
