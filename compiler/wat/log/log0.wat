(module
  (type (;0;) (func (param i64 i64)))
  (type (;1;) (func (param i32 i32)))
  (import "zink" "log0" (func (;0;) (type 0)))
  (func (;1;) (type 1) (param i32 i32)
    local.get 0
    i64.extend_i32_u
    local.get 1
    i64.extend_i32_u
    call 0)
  (memory (;0;) 16)
  (global (;0;) i32 (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "log" (func 1))
  (export "__data_end" (global 0))
  (export "__heap_base" (global 1)))
