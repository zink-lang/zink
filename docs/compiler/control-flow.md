# Control Flow

EVM doesn't have instructions for the custom conrol flows, however
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

```
PUSH1 0x00       // Load the params at 0x00
CALLDATALOAD

ISZERO           // if is zero, jump to 0x0c, the else block.
PUSH1 0x0c
JUMPI

PUSH1 0x07       // if is non-zero, enters the if block.
                 // push 0x07 on stack.

PUSH1 0x0f       // jump to the end of the else block.
JUMP

JUMPDEST         // destination of the else block, push 0x08
PUSH1 0x08       // on stack.


JUMPDEST         // the end of the else block.


PUSH1 0x00       // pack the result and return...
MSTORE
PUSH1 0x20
PUSH1 0x00
RETURN
```
