(module
  (func (export "main") (param i32) (param i32) (result i32)
    (call $add (local.get 0) (local.get 1))
  )

  (func $add (param i32 i32) (result i32)
    (local.get 0)
    (local.get 1)
    (i32.add)
  )
)
