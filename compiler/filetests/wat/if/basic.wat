(module
  (func (export "basic") (param i32) (result i32)
    (if (local.get 0) (then (nop)))
    (local.get 0)
  )
)
