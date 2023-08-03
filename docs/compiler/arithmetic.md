# Arithmetic

The arithmetic operators have a lot of differences between WASM and EVM bytecode,
all of the operand requires the order of the stack are reserved...

## Sub, Div, Mod and Bitwise Operand

```wasm
i32.const 2       ;; PUSH1 0x02
i32.const 1       ;; PUSH1 0x01
sub               ;; SWAP1
                  ;; SUB
```

This `SWAP1` sticks to all of these reversed order instructions, will introduce
macros to optimize it in `v0.3.0`.

## Comparison

The order of comparison are reversed as well, but however, they are paired!

```wasm
i32.const 1    ;; PUSH1 0x01
i32.const 0    ;; PUSH1 0x00
gt             ;; LT
```

This is insane, but works perfectly, don't think too much about it, focus
on if the output is `0` or `1` ;)
