# Fibonacci

```rust
//! fibonacci example.
#![no_std]

// for the panic handler.
#[cfg(not(test))]
extern crate zink;

/// Calculates the nth fibonacci number.
#[no_mangle]
pub extern "C" fn fibonacci(n: usize) -> usize {
    recursion(n)
}

/// Calculates the nth fibonacci number using recursion.
#[no_mangle]
pub extern "C" fn recursion(n: usize) -> usize {
    if n < 2 {
        n
    } else {
        recursion(n - 1) + recursion(n - 2)
    }
}
```

A recursion example, complex in bytecode

```wasm
(module
  (type (;0;) (func (param i32) (result i32)))
  (func (;0;) (type 0) (param i32) (result i32)
    local.get 0
    call 1)
  (func (;1;) (type 0) (param i32) (result i32)
    (local i32)
    local.get 0
    i32.const 2
    i32.ge_u
    if  ;; label = @1
      loop  ;; label = @2
        local.get 0    ;; 1
        i32.const 1    ;; 2
        i32.sub        ;; 1
        call 1         ;; 1
        local.get 1    ;; 2
        i32.add        ;; 1
        local.set 1    ;; 0
        local.get 0    ;; 1
        i32.const 2    ;; 2
        i32.sub        ;; 1
        local.tee 0    ;; 1
        i32.const 1    ;; 2
        i32.gt_u       ;; 1
        br_if 0 (;@2;) ;; 2 -> 0
      end
    end
    local.get 0
    local.get 1
    i32.add))
```

A more complex implementation of locals ( params + local variables) is introduced in this example,
control flow `br_if` and `loop` are compiled as well.

```
600035586010565b60005260206000f35b906000816002600190031015603d575b8160019003586010565b8101905081600290038092506001106020575b8181019150509060040156
```
