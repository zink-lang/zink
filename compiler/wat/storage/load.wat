(module
  (type (;0;) (func (param i64) (result i64)))
  (type (;1;) (func (param i64 i64)))
  (import "evm" "sload" (func (;0;) (type 0)))
  (import "evm" "sstore" (func (;1;) (type 1)))
  (func (type 0) (param i64) (result i64)
        i64.const 0
        local.get 0
        call 1
        i64.const 0
        call 0))
