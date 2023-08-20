# Building

This section describes everything required to build and run zink.

## Prerequisites

Before we can actually build Zink, we'll need to make sure these things are installed first.

### The Rust toolchain

[Install the Rust toolchain here][rust]. This includes `rustup`, `cargo`, `rustc`, etc...

### Add target `wasm32-unknown-unknown`

```
rustup target add wasm32-unknown-unknown
```

This is required for compiling our rust projects to wasm.

## Build the `zinkc` CLI

```bash
cd cli
cargo b -p zinkup --release --features zinkc
```

The built executable will be located at `target/release/zinkc`.

## Build examples

```bash
cd examples
cargo b --release
```

The built wasm binaries will be localted at `examples/target/wasm32-unknown-unknown/realease/*.wasm`,
then, you can you `zinkc` to compile them to EVM bytecode!

[rust]: https://www.rust-lang.org/tools/install
