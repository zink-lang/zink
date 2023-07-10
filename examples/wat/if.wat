(module
  (type (;0;) (func (param i64 i64) (result i64)))
  (func (;0;) (type 0) (param i64 i64) (result i64)
    local.get 0
    local.get 1
    local.get 0
    local.get 1
    i64.gt_u
    select)
  (memory (;0;) 16)
  (global (;0;) i32 (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "main" (func 0))
  (export "__data_end" (global 0))
  (export "__heap_base" (global 1)))
