# Arithmetic

The arithmetic operators are different between WASM and EVM bytecode:

| INPUT   | WASM    | STACK   | EVM | STACK   |
| ------- | ------- | ------- | --- | ------- |
| `a + b` | i32.add | `a` `b` | ADD | `a` `b` |
| `a - b` | i32.sub | `a` `b` | SUB | `b` `a` |
