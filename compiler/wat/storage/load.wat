(module
  (type (;0;) (func (param i32) (result i32)))
  (type (;1;) (func (param i32 i32)))
  (import "evm" "sload" (func (;0;) (type 0)))
  (import "evm" "sstore" (func (;1;) (type 1)))
  (func (type 0) (param i32) (result i32)
        i32.const 0
        local.get 0
        call 1
        i32.const 0
        call 0))
