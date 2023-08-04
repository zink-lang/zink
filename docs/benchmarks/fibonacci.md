# Fibonacci

Benchmarks for fibonacci.

## Recursion

| fib(n) | Zink | Solidity@0.8.21 |
| ------ | ---- | --------------- |
| 0      | 110  | 605             |
| 1      | 110  | 605             |
| 2      | 262  | 3636            |
| 3      | 414  | 6667            |
| 4      | 718  | 12729           |
| 5      | 1174 | 21822           |

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
  if (n <= 1) {
    return n;
  } else {
    return this.fib(n - 1) + this.fib(n - 2);
  }
}
```

Vyper is not included since it doesn't support cyclic function call :(
