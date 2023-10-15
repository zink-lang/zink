(module
  (type (;0;) (func (param i32) (result i32)))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (result i32)))
  (import "evm" "sstore" (func (;0;) (type 1)))
  (import "evm" "sload" (func (;1;) (type 0)))
  (func (;2;) (type 0) (param i32) (result i32)
    i32.const 0
    local.get 0
    call 0
    i32.const 0
    call 1)
  (func (;3;) (type 2) (param i32)
    i32.const 0
    local.get 0
    call 0)
  (func (;4;) (type 3) (result i32)
    i32.const 0
    call 1)
  (global (;0;) i32 (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (export "set_and_get" (func 2))
  (export "set" (func 3))
  (export "get" (func 4))
  (export "__data_end" (global 0))
  (export "__heap_base" (global 1)))
