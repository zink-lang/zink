(module
  (func (export "br_if") (param i32) (result i32)
    loop ;; label = @1
      i32.const 1
      local.get 0
      i32.gt_s
      br_if 0 (;1;)
    end

    i32.const 7
  )
)
