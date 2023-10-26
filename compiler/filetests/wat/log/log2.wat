(module
  (type (;0;) (func))
  (type (;3;) (func (param i32 i32 i32 i32 i32 i32)))
  (import "evm" "log2" (func (;0;) (type 1)))
  (import "env" "memory" (memory (;0;) 17))
  (func (;1;) (type 0)
    i32.const 1048576
    i32.const 4
    i32.const 1048580
    i32.const 4
    i32.const 1048584
    i32.const 4
    call 0)
  (global (;0;) i32 (i32.const 1048588))
  (global (;1;) i32 (i32.const 1048592))
  (export "log2" (func 1))
  (export "__data_end" (global 0))
  (export "__heap_base" (global 1))
  (data (;0;) (i32.const 1048576) "Pingpongping"))
