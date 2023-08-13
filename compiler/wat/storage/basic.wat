(module
  (type (;0;) (func (param i64) (result i64)))
  (type (;1;) (func (param i64 i64)))
  (type (;2;) (func (param i64)))
  (type (;3;) (func (result i64)))
  (import "zink" "sstore" (func (;0;) (type 1)))
  (import "zink" "sload" (func (;1;) (type 0)))
  (func (;2;) (type 0) (param i64) (result i64)
    i64.const 0
    local.get 0
    call 0
    i64.const 0
    call 1)
  (func (;3;) (type 2) (param i64)
    i64.const 0
    local.get 0
    call 0)
  (func (;4;) (type 3) (result i64)
    i64.const 0
    call 1)
  (memory (;0;) 16)
  (global (;0;) i32 (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "set_and_get" (func 2))
  (export "set" (func 3))
  (export "get" (func 4))
  (export "__data_end" (global 0))
  (export "__heap_base" (global 1)))
