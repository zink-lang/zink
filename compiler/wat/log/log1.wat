(module
  (type (;0;) (func))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func (param i32 i32 i32 i32)))
  (import "evm" "log0" (func (;0;) (type 1)))
  (import "evm" "log1" (func (;1;) (type 2)))
  (import "env" "memory" (memory (;0;) 17))
  (func (;2;) (type 0)
    i32.const 1048576
    i32.const 4
    call 0)
  (func (;3;) (type 0)
    i32.const 1048576
    i32.const 4
    i32.const 1048580
    i32.const 4
    call 1)
  (global (;0;) i32 (i32.const 1048584))
  (global (;1;) i32 (i32.const 1048592))
  (export "log0" (func 2))
  (export "log1" (func 3))
  (export "__data_end" (global 0))
  (export "__heap_base" (global 1))
  (data (;0;) (i32.const 1048576) "Pingpong"))
