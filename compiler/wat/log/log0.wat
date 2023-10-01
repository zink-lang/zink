(module
  (type (;0;) (func (param i32)))
  (type (;1;) (func))
  (import "evm" "log0" (func (;0;) (type 0)))
  (import "env" "memory" (memory (;0;) 17))
  (func (;1;) (type 1)
    i32.const 1048576
    call 0)
  (global (;0;) i32 (i32.const 1048580))
  (global (;1;) i32 (i32.const 1048592))
  (export "log0" (func 1))
  (export "__data_end" (global 0))
  (export "__heap_base" (global 1))
  (data (;0;) (i32.const 1048576) "Ping"))
