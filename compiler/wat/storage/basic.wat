(module
  (type (;0;) (func (param i64) (result i64)))
  (type (;1;) (func (param i64 i64)))
  (import "evm" "sstore" (func (;0;) (type 1)))
  (import "evm" "sload" (func (;1;) (type 0)))
  (func (;2;) (type 0) (param i64) (result i64)
    i64.const 0
    local.get 0
    call 0
    i64.const 0
    call 1))
