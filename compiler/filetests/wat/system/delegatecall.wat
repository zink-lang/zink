(module
  (type (;0;) (func (param i64 i32 i64 i64 i64 i64) (result i32)))
  (import "evm" "delegatecall" (func (;0;) (type 0)))
  (func (export "main") (result i32)
    i64.const 50000
    i32.const 0
    i64.const 0
    i64.const 0
    i64.const 0
    i64.const 0
    call 0)
)
