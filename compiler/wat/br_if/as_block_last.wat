;;! target = "evm"
(module
  (func (export "as-block-last") (param i32)
    (block
     (call $dummy)
     (call $dummy)
     (br_if 0 (local.get 0))
    )
  )
  (func $dummy)
)
