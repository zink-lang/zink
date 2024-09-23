# Storage

The storage keys in Zink is slot based, for example, the first detected 
storage in compilation will be using `0` as storage key.

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

## Mapping

Mapping keys are generated via `keccak256(slot, key)`

## Array

Similar to mappings, but the keys will be using `u32` / `u64` for indexing due to the optimization 
on the wasm side in the zink compiler, which means, the max size of an array is `max(u64)`. 
