# Storage

The storage keys in Zink is number based for reducing the bytecode size and the calculation 
in, for example, the first detected storage in compilation will be using `0` as storage key.

```solidity
// Loading storage at 0
PUSH0
SLOAD

// Loading storage at 1
PUSH1 0x01
SLOAD
```

## Key-Value

As mentioned above, all key-value pairs follows using number as storage key, however, the value
will be limited with 32 bytes, dynamic value like string is currently not supported.

## Array

With provided storage index number `m`, the index of an element `n` in `Array` will appended to the 
storage key, for example, the storage index of array `A` is `0x00`. 

```
// storage index
0x00

// i32 as indexing
0x0000   // element 0x00
0x0001   // element 0x01
0x00ff   // element 0xff

// i64 as indexing
0x00ffff // element 0xffff
```

Indexing greater than `max(i64)` is currently not supported due to the limitaion of current 
optimization for WASM in zink compiler.

## Mapping

Mappings is similar to arrays, but keys and values will be distributed in different suffix.

```
// storage index
0x42

// i32 as indexing
0x420000      // key of element 0
0x4200ff      // value of element 0

// i64 as indexing
0x42000000    // key of element 0
0x420000ff    // value of element 0
```

The max size of `Mapping` is `max(i64)` as well at the moment for the same reason.
