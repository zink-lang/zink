# Creating Zink Project

The Zink projects are based on the [cargo projects][cargo], you can create
a cargo project and specify 

```toml
# ...

[lib]
crate-type = [ "cdylib" ]

[dependencies]
zink = "0.1.0"

# ...
```

in your `Cargo.toml` on your own, or just use zink's package manager [elko][elko] 
to generate it:


```shell
elko new my-awesome-contract
Created zink project `my-awesome-contract`
```

open `my-awesome-contract/src/lib.rs`

```rust
//! my-awesome-project
#![no_std]

// For the panic handler.
#[cfg(not(test))]
extern crate zink;

/// Adds two numbers together.
#[no_mangle]
pub extern "C" fn addition(x: u64, y: u64) -> u64 {
    x + y
}
```

you'll see a standard `WASM` library in rust:

1. `#![no_std]` means we don't need the std library in this project.
2. `extern crate zink` is for importing the panic handler from library `zink` for this project.
3. `#[no_mangle]` is for exporting function `addition` to WASM, and this will be one the methods
of your contracts.

[cargo]: https://doc.rust-lang.org/cargo/reference/manifest.html
[elko]: /cli/elko.html
