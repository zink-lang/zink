# Creating Zink Project

For creating a zink project, we need to install the zink toolchain `zinkup`
from `crates.io` first, the package manager `elko` will be installed along
with other tools:

```bash
cargo install zinkup
elko -h
```

Now, let's create a zink project:

```bash
elko new my-awesome-contract
Created zink project `my-awesome-contract`
```

the Zink projects are based on the [cargo projects][cargo], you can install
dependencies you need with `cargo`, the basic `Cargo.toml` will be like:

```toml
# ...

[lib]
crate-type = [ "cdylib" ]

[dependencies]
zink = "0.1.0"

# ...
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
