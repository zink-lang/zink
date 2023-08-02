# Calls

> TODO: move this chapter to locals.

The function calls are compiled by order in zink, the first call
in order (index 0) will be the main function ( to be updated ).

For passing params to callee functions, zink will store the arguemnts
to memory as locals and pass the memory pointer to the callee functions
while compiling, for example:

```
I'm example
```
