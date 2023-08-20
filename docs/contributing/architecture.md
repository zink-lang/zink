# Architecture of Zink

## Compiler

The path of the compiler crate is `/compiler`, as its name, it's the zink compiler
`zinkc`, currently just a wrapper of `zingen`, the codegen library.

So if you want to contribute to the compiler, the code inside `/compiler` and `/codegen`
will be interested for you!

## Zink

Located at `/zink`, it is a rust library for compiling cargo project to zink program
with provided apis, `selector`, `events`...any sugar or asm macro for zink will be
embedded in this library.

## Test utils

`/zint` is the testing library for zink projects, it is currently just a wrapper
of `evm`, we need really a lot of features in this in `v0.3.0`.
