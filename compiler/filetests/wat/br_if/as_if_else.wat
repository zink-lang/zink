;;! target = "evm"
(module
  (func (export "as-if-else") (param i64) (result i64)
    (if (result i64) (local.get 0) (i64.const 0) (i64.gt_s)
      (then (i64.const 7))
      (else (i64.const 8))
    )
  )
)