(module
    (func (param i32) (param i32) (result i32)
    (local.get 0)
    (local.get 1)
    (i32.add)
    )
)

 ;; BYTECODE          MNEMONIC         STACK                  ACTION
 ;; 60 00          // PUSH1 0x00       // [0x00]              //
 ;; 35             // CALLDATALOAD     // [number1]           // Store the first 32 bytes on the stack
 ;;
 ;; 60 20          // PUSH1 0x20       // [0x20, number1]     //
 ;; 35             // CALLDATALOAD     // [number2, number1]  // Store the second 32 bytes on the stack
 ;;
 ;; 01             // ADD              // [number2+number1]   // Take two stack inputs and add the result
 ;;
 ;; 60 00          // PUSH1 0x00       // [0x0, (n2+n1)]      //
 ;; 52             // MSTORE           // []                  // Store (n2+n1) in the first 32 bytes of memory
 ;;
 ;; 60 20          // PUSH1 0x20       // [0x20]              //
 ;; 60 00          // PUSH1 0x00       // [0x00, 0x20]        //
 ;; f3             // RETURN           // []                  // Return the first 32 bytes of memory
