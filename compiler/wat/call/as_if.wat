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
      i32.const 39
      local.set 1
    end
    local.get 0
    local.get 1
    i32.add))
