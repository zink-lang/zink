# Fibonacci

Benchmarks for fibonacci.

## Recursion

| fib(n) | Zink | Solidity@0.8.21 |
| ------ | ---- | --------------- |
| 0      | 110  | 614             |
| 1      | 110  | 614             |
| 2      | 262  | 1322            |
| 3      | 414  | 2030            |
| 4      | 718  | 3446            |
| 5      | 1174 | 5570            |

`zink` implementation in rust:

```rust
//! Zink fibonacci recursion

#[no_mangle]
pub extern "C" fn fib(n: usize) -> usize {
    if n < 2 {
        n
    } else {
        recursion(n - 1) + recursion(n - 2)
    }
}
```

`solidity` implementation:

```sol
/**
 * Solidity fibonacci recursion
 **/

function fib(uint n) public view returns (uint) {
  if (n < 2) {
    return n;
  } else {
    return fib(n - 1) + fib(n - 2);
  }
}
```

Vyper is not included since it doesn't support cyclic function call :(
