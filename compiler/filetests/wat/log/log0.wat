(module
  (type (;0;) (func))
  (type (;1;) (func (param i32 i32)))
  (import "evm" "log0" (func (;0;) (type 1)))
  (import "env" "memory" (memory (;0;) 17))
  (func (;2;) (type 0)
    i32.const 1048576
    i32.const 4
    call 0)
  (data (;0;) (i32.const 1048576) "Ping"))
