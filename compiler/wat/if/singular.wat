(module
  (func (export "singular") (param i32) (result i32)
    (if (local.get 0) (then (nop)))
    (if (local.get 0) (then (nop)) (else (nop)))
    (if (result i32) (local.get 0) (then (i32.const 7)) (else (i32.const 8)))
  )
)

;; WAT            // MNEMONIC                       // ACTION
;;
;; local.get 0    // PUSH1 0x00 CALLDATALOAD
;; IF             // JUMPI $LABEL_ELSE              // If false, jump to $LABEL_ELSE
;; NOP
;; END            // JUMPDEST                       // Set the end as a jump destination
;;
;;
;; local.get 0    // PUSH1 0x00 CALLDATALOAD
;; IF             // JUMPI $LABEL_ELSE              // If false, jump to $LABEL_ELSE
;; NOP
;; ELSE           // JUMPDEST                       // Set the else as a jump destination
;; NOP
;; END            // JUMPDEST                       // Set the end as a jump destination
