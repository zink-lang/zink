(module
  (type (;0;) (func (param i64 i64) (result i64)))
  (func $if_else (type 0) (param i64 i64) (result i64)
    local.get 0
    local.get 1
    local.get 0
    local.get 1
    i64.gt_u
    select))
