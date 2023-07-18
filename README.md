# Zink 

> This project is still under development, plz DO NOT use it in production.

[![zink][version-badge]][version-link]
[![ci][ci-badge]][ci-link]
[![telegram][telegram-badge]][telegram-group] 

The Zink project mainly provides an optimizing compiler `zinkc` which can compile WASM 
to the EVM bytecode with optimizations, the source code of your smart contract could be 
any language you like! 

```mermaid
flowchart LR
    R{{Rust}} --> W(WebAssembly)
    O[...] --> W
    W --> Z{Zink Compiler}
    Z --> V[(EVM)]
```

## Features

Here we highly recommand you to choose `rust` as the language of your smart contracts 
which will unlock all of the following features:

- **Safe**: `rustc` is wathcing you! Furthermore, after compiling your rust code into WASM, 
`zinkc` will precompute all of the stack and memory usages in your contracts to ensure they
are safe in EVM bytecode as well!

- **High Performance**: The optimizations are provided by the three of `rustc`, `wasm-opt` 
and `zinkc`, your contracts will have the smallest size with **strong performance** in EVM 
bytecode at the end! More details plz check [Optimizations](./docs/optimizations.md).

- **Compatible**: All of the `no_std` libraries in rust are your libraries, futhermore, you 
can use your solidity contracts as part of your zink contracts and your zink contracts as 
part of your solidty contracts :)

- **Easy Debugging**: Developing your smart contracts with only one programming language! 
zink will provide everything you need for developing your contracts officially based on the 
stable projects in rust like the `foundry` tools.


## LICENSE

GPL-3.0-only

[telegram-badge]: https://img.shields.io/badge/telegram-blue?logo=telegram 
[telegram-group]: https://t.me/+6oZpbwxlVD81OGQ1
[version-badge]: https://img.shields.io/crates/v/zink
[version-link]: https://docs.rs/zink/latest/zink/
[ci-badge]: https://img.shields.io/github/actions/workflow/status/clearloop/zink/CI.yml
[ci-link]: https://github.com/clearloop/zink/actions/workflows/main.yml
[rustc-codegen]: https://doc.rust-lang.org/rustc/codegen-options/index.html
[wasm-opt]: https://github.com/WebAssembly/binaryen#binaryen-optimizations
