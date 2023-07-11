(module
  (func (export "basic") (param i32) (result i32)
    (if (local.get 0) (then (nop)))
    (i32.const 8)
  )
)
