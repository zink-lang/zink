# Calls

The function calls are compiled by order in zink, the first call
in order (index 0) will be the main function ( to be updated ).

## Functions

There are only internal functions and external functions in zink
project for now.

### Internal Functions

The parameters of the internal functions will be queued to the
[locals](./locals.md) of the them and taking the first `n` stack
for storing them.

### External Functions

Same as internal functions, will be updated once have the design
of selector in [v0.2.0][v0.2.0]

### Extended Functions

We have also introduces extended functions inside the compiler
for complete the difference between EVM bytecode and WASM,
see the implementation [select](./control-flow.md#select) as example.

### Main Function

You may never meet this because it is embedded in the compiled
bytecode, but it is the entry of zink programs.

It takes parameters from the `calldata` by order, for loading
32-byte parameters, it will process

```yul
// parameter 1
PUSH1 0x00
calldataload

// parameter 2
PUSH1 0x20
calldataload
```

## Layout

Each function in zink is started with `JUMPDEST` in the layout
of the bytecode for the insane jumping...

Each function call's stack starts with `PC` which stores the last active
program counter for the program for jumping back to the main process since
the callee functions could be called by any functions but not only one.

There is a tricky problem that **how to detect the last pc before jumping**,
for solving this, `zinkc` registers the original `PC` to the jump table when
meeting jumps and relocates them after compiling all functions.

### Example Addition

```wasm
(module
  (func (export "main") (param i32) (param i32) (result i32)
    (call $add (local.get 0) (local.get 1))
  )

  (func $add (param i32 i32) (result i32)
    (local.get 0)
    (local.get 1)
    (i32.add)
  )
)
```

Let's assume we are calling an `add` function with parameters `1, 1` and
now we are at the first byte right before it:

```yul
/* 0x00 */  PUSH1 0x01    // push the first parameter on the stack
/* 0x02 */  PUSH1 0x01    // push the second  parameter on the stack
/*      */                //
/*      */                //
/* 0x04 */  pc            // the first byte before calling the callee function
/*      */                //
/*      */                //
/* 0x05 */  PUSH1 0x42    // This 0x42 will be reloacted by `zinkc`
/*      */                //
/*      */                //
/* 0x07 */  jump          // jump to the callee function
/*      */                //
/*      */                //
/* 0x08 */  jumpdest      // the pc for jumping back from the callee function.
/*      */                //
/*      */                // the rest logic of the main process.
/*      */                //
/*      */                //
/* 0x42 */  jumpdest      // the first byte of the callee function
/*      */                //
/*      */                //
/* 0x43 */  add           // for the current stack: [PC, 0x02]
/*      */                //
/*      */                //
/* 0x44 */  SWAP1         // shift the stored PC to the top of the stack
/*      */                //
/*      */                //
/* 0x45 */  PUSH1 0x04    // the jumpdest is the original pc + 4 bcz we have
/* 0x47 */  add           // `push1`, `0x42`, `jump`, `jumpdest` queued after
/*      */                // `pc`.
/*      */                //
/*      */                //
/* 0x48 */  jump          // This 0x07 will be reloacted by `zinkc`

```

[v0.2.0]: https://github.com/clearloop/zink/milestone/2
