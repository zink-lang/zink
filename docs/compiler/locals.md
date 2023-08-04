# Locals

There are two usages of locals in zink.

1. The parameters of functions are loaded as locals.
2. local defined variables in functions.

|       | fn params   | local variables |
| ----- | ----------- | --------------- |
| stack | locals[..n] | locals[n..]     |

## Function Parameters

Let's go through the `add-two` example:

```wasm
(module
    (func (param (;0;) i32) (param (;1;) i32) (result i32)
    (local.get 0)
    (local.get 1)
    (i32.add)
    )
)
```

`(param (;0;) i32)` and `(param (;1;) i32)` will be pushed to the function
locals with index `0` and index `1` with their type `i32` recorded.

`zinkc` gets the defined local at index `0` when reaching `(local.get 0)`,
at index `1` for `(local.get 1)`, for example, for `(local.get 0)`, it will
be translated to:

```yul
push1 0x00
calldataload
```

for `(local.get 1)`, that would be

```yul
push1 0x20
calldataload
```

You may have problem why we `PUSH1 0x20` while getting local at index `1`, the
anwser is that this offset is calculated by the size of the parameters.

The `CALLDATALOAD` operator has stack input `i` and output `data[i]` while `data[i]`
is a `32-byte` value starting from the given offset of the calldata, so the minimal
size of our types will be `32-byte`, therefore, we align all types sizes to `32-byte`
in `zinkc`.

> WARN: We don't care about the originals offset of the parameters in WASM bcz we will
> serialize them into our locals and calcualte the offsets on our own when need anyway.

| type  | size     | aligned size |
| ----- | -------- | ------------ |
| `i32` | `[u8;4]` | `[u8;32]`    |
| `i64` | `[u8;8]` | `[u8;32]`    |

It is a waste of resources but sadly this is also how EVM works ))

## Local Variables

The locals variables will take the stack items right after the function
parameters, for example:

```wasm
(func (result i32)
  (local i32)
  i32.const 42
  local.set 0
  local.get 0)
```

In the program above, we set and get `42` to local variable `0` and returns it.

While compiling this function, `zinkc` will push local variable `0` on the stack
with an initializing value `0` first, getting with `dup` and setting with `swap`
and `drop`.

```yul
PUSH1 0x00       // initializing value 0 for local 0
                 //
                 //
PUSH1 0x28       // push value 42 on stack, the current stack is [0, 42]
                 //
                 //
SWAP1            // swap the value on the top of the stack to local 0 and
DROP             // drop the previous value of it for cleaning stack, `swapn`
                 // is calculated by `zinkc`. current stack: [42]
                 //
                 //
DUP1             // dup the value of local 0 and push it on the top of the
                 // stack, `dupn` is calculated by `zinkc`.
                 //
                 //
DROP             // clean the stack before returning results.
```

As we can see, the usages of `get` and `set` is verbose with `swap` and `dup`,
it is for adapting any usages but not necessary for all of them, however, we
will introduce optimizer for this in `v0.4.0`!
