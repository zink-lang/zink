(module
  (type (;0;) (func))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param i32)))
  (type (;4;) (func (result i32)))
  (import "evm" "sstore" (func (;0;) (type 1)))
  (import "evm" "sload" (func (;1;) (type 2)))
  (import "zinkc" "emit_abi" (func (;2;) (type 1)))
  (import "env" "memory" (memory (;0;) 17))
  (func (;3;) (type 2) (param i32) (result i32)
    local.get 0
    i32.const 0
    call 0
    i32.const 0
    call 1)
  (func (;4;) (type 0)
    i32.const 1048576
    i32.const 34
    call 2)
  (func (;5;) (type 3) (param i32)
    local.get 0
    i32.const 0
    call 0)
  (func (;6;) (type 0)
    i32.const 1048610
    i32.const 18
    call 2)
  (func (;7;) (type 4) (result i32)
    i32.const 0
    call 1)
  (func (;8;) (type 0)
    i32.const 1048628
    i32.const 10
    call 2)
  (global (;0;) i32 (i32.const 1048638))
  (global (;1;) i32 (i32.const 1048640))
  (export "set_and_get" (func 3))
  (export "set_and_get_selector" (func 4))
  (export "set" (func 5))
  (export "set_selector" (func 6))
  (export "get" (func 7))
  (export "get_selector" (func 8))
  (export "__data_end" (global 0))
  (export "__heap_base" (global 1))
  (data (;0;) (i32.const 1048576) "0b7365745f616e645f67657401036933320373657401036933320367657400"))
