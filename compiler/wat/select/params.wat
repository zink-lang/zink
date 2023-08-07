(module
  (func $if_else (param i64 i64) (result i64)
    local.get 0
    local.get 1
    i64.gt_u
    select))
