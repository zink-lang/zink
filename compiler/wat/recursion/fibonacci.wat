(module
  (type (;0;) (func (param i32) (result i32)))
  (func (;0;) (type 0) (param i32) (result i32)
    local.get 0
    call 1)
  (func (;1;) (type 0) (param i32) (result i32)
    (local i32)
    local.get 0
    i32.const 2
    i32.ge_u
    if  ;; label = @1
      loop  ;; label = @2
        local.get 0
        i32.const 1
        i32.sub
        call 1
        local.get 1
        i32.add
        local.set 1
        local.get 0
        i32.const 2
        i32.sub
        local.tee 0
        ;; local.set 0
        ;; local.get 0
        i32.const 1
        i32.gt_u
        br_if 0 (;@2;)
      end
    end
    local.get 0
    local.get 1
    i32.add)
  (memory (;0;) 16)
  (global (;0;) i32 (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "fibonacci" (func 0))
  (export "recursion" (func 1))
  (export "__data_end" (global 0))
  (export "__heap_base" (global 1)))
