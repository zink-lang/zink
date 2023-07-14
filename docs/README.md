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


## Function Calls

### Calling convention

There we two ways to handle the calling convention, for storing PC 

1. Store the PC

few arguments     -> store the PC on stack.
lots of arguemnts -> store the PC in reserved memory.


2. Retrive the PC

stack  -> swap the parameters and the PC
memory -> read from reserved memory


3. Jump back to the caller

stack  -> swap the results and the PC
       -> dup the PC and pop in caller function
memory -> load PC from memory

