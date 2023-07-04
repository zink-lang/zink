(module
  (type (;0;) (func (param i64 i64) (result i64)))
  (func $main (type 0) (param i64 i64) (result i64)
    local.get 1
    local.get 0
    i64.add)
  (memory (;0;) 16)
  (global $__stack_pointer (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (global (;2;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "main" (func $main))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2)))
