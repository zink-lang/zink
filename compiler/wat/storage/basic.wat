(module
  (type (;0;) (func (param i32) (result i32)))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (result i32)))
  (func $set_and_get (type 0) (param i32) (result i32)
    i32.const 1048572
    local.get 0
    i32.store
    i32.const 1048576
    local.get 0
    i32.store
    i32.const 1048576
    i32.load)
  (func $set (type 1) (param i32)
    i32.const 1048572
    local.get 0
    i32.store
    i32.const 1048576
    local.get 0
    i32.store)
  (func $get (type 2) (result i32)
    i32.const 1048576
    i32.load)
  (memory (;0;) 17)
  (global (;0;) i32 (i32.const 1048580))
  (global (;1;) i32 (i32.const 1048592))
  (export "memory" (memory 0))
  (export "set_and_get" (func $set_and_get))
  (export "set" (func $set))
  (export "get" (func $get))
  (export "__data_end" (global 0))
  (export "__heap_base" (global 1)))
