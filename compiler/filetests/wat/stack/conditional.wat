(module
    (func $conditional (param i32) (result i32)
        (if (i32.eq (local.get 0) (i32.const 0))
            (then
                (i32.const 42)
                (drop)
            )
            (else
                (i32.const 1)
            )
        )
        (i32.const 1)
    )
    (export "conditional" (func $conditional))
)