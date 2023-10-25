(module
    (func (result i64)
        (local $foo i64)
        (local $bar i64)

        (i64.const 10)
        (local.tee $foo)

        (i64.const 20)
        (local.set $bar)

        (local.get $bar)
        i64.add
    )
)
