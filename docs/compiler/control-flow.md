# Control Flow

EVM doesn't have instructions for the custom control flows, however
zink implements them with `JUMPI` and `JUMP`, which includes:

- `if`
- `block`
- `loop`
- `else`
- `select`
- `br`
- `br_if`
- `br_table`

## If-Else

The beginning of an if construct with an implicit then block, plus and else block.

The basic logic is, if non-zero, enter the if block, otherwise jump to the else block
or the end of the if condition.

```wasm
(if (result i32)
  (local.get 0)
  (then (i32.const 7))
  (else (i32.const 8)))
```

The expected result is

| Input | Result |
| ----- | ------ |
| 1     | 7      |
| 0     | 8      |

so in the compiled bytecode, the code snippet above will be

```yul
PUSH1 0x00       // Load the params at 0x00
calldataload

iszero           // if is zero, jump to 0x0c, the else block.
PUSH1 0x0c
jumpi

push1 0x07       // if is non-zero, enters the if block.
                 // push 0x07 on stack.

PUSH1 0x0f       // jump to the end of the else block.
jump

jumpdest         // destination of the else block, push 0x08
push1 0x08       // on stack.


jumpdest         // the end of the else block.


PUSH1 0x00       // pack the result and return...
mstore
PUSH1 0x20
PUSH1 0x00
return
```

## Select

The `select (0x1B)` instruction comes from WebAssembly, it selects
one of its first two operands based on whether its third operand is
zero or not.

Simple rust conditions in rust will be compiled to `select`.

```rust
pub extern "C" fn if_else(x: u64, y: u64) -> u64 {
    if x > y {
        x
    } else {
        y
    }
}
```

As we can see in the example above, we simply returns the bigger
number from the 2 parameters, the logic in the two blocks of
`if-else` is explicit direct, that will be compiled to `select`.

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

Since EVM doesn't have instruction like `select`, we need to provide
it ourselves in our implementation like an external function, if zero
pop the value on the top of the stack.

```rust
const SELECT: [OpCode; 6] = [
    OpCode::JUMPDEST,
    OpCode::POP,
    OpCode::PUSH1,
    OpCode::Data(0x06),
    OpCode::ADD,
    OpCode::JUMP,
];
```

In the compiled code, we need to combine this function `select` with
`jumpi` in EVM.

```yul
PUSH1 0x00      // Load the parameters.
calldataload

PUSH1 0x20
calldataload

PUSH1 0x00
calldataload

PUSH1 0x20
calldataload

lt               // Compiled to `lt` because of the result of this
                 // instruction is oppsited between EVM and WASM.

pc
swap2            // shift
swap1
PUSH1 0x1c
jumpi            // `jumpi` for the if condition.
JUMPDEST

PUSH1 0x00       // Returns the value.
mstore
PUSH1 0x20
PUSH1 0x00
return

JUMPDEST         // Function select starts here.
pop
PUSH1 0x06
add
jump
```
