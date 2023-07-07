# The Zink Book


## Validation

e.g. How Zink Compiler helps you writing your EVM smart contracts ;)


#### `0x35` - CALLDATALOAD

1. validate the function signatures
2. validate the stack usages


## Optimizations


#### StackCompressor

The max limit of the defined local variables is 16 due to there is a hard limit 
of 16 slots for reaching down the expression stack of EVM.
