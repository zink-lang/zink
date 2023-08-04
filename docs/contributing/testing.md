# Testing

This section describes how to run Zink's tests and add new tests.

Before continuing, make sure you can build Zink successfully. Can't
run the tests if you can't build it!

## Running All Tests

```bash
cargo test --all
```

## Adding New Tests for the Compiler

At the current stage, we are lack of the tests of the compiler,
the tests of it are gathered in `compiler/tests`.

Each file under `compiler/tests` are named by instruction, for
example `add.rs`, it includes the tests related to instruction
`ADD`.

For adding a new test for `compiler/tests/add.rs`, we need to write
a wasm program for it first, for example, the wasm program of the
`params` test of `ADD` is located at `compiler/wat/i32add/params.wat`.

```wasm
(module
    (func (param i32) (param i32) (result i32)
    (local.get 0)
    (local.get 1)
    (i32.add)
    )
)
```

In `compiler/tests/add.rs`:

```rust
#[test]
fn params() -> Result<()> {
    let bytecode = common::load("i32add", "params")?;

    // add(1, 2)
    let input = [1.to_bytes32(), 2.to_bytes32()].concat();
    let info = EVM::run(&bytecode, &input);

    assert_eq!(info.ret, [3.to_bytes32()].concat());
    Ok(())
}
```

We use `common::load("i32add", "params")` to load wat file to EVM bytecode
from `compiler/tests/wat/i32add/params.wat`, execute with the `EVM` provided
by `zint`, compare if the result is as expected, that's it!
