<img align="right" width="150" height="150" top="100" src = "https://avatars.githubusercontent.com/u/138247979?s=400&u=cbf4b9e9da048899a947f08d92e030806d5bd50b&v=4"/>

# The Zink Language

> [!CAUTION]
>
> This project is still under active development, please DO NOT use it in production.

[![zink][version-badge]][version-link]
[![ci][ci-badge]][ci-link]
[![telegram][telegram-badge]][telegram-group]

- **Safe**: `rustc` monitors your code!

- **Efficient**: Efficient EVM bytecode from `rustc`, `wasm-opt`, and `zinkc`.

- **Modular**: Upload and download your contract components via `crates.io`.

- **Rusty**: All of the rust tools are available for your contracts!

```rust
//! ERC20 Example (In Development)
#[zink::contract]
pub struct ERC20;

#[zink::calls]
impl ERC20 {
  /// VMs that zink supports
  pub fn support() -> [zink::String; 4] {
    ["EVM", "WASM", "RISC-V", "...OTHER_VMS"]
  }
}

#[zink::interface]
impl ERC20 for ERC20 {
  fn name() -> zink::String {
    "Zink Language".to_string()
  }
}
```

Run `cargo install zinkup` to install the toolchain! See [book](https://zink-lang.org/) for more details.

## Testing & Development

| Command    | Description            |
| ---------- | ---------------------- |
| `cargo cc` | Clippy all packages    |
| `cargo tt` | Run all tests          |
| `cargo be` | Build all examples     |
| `cargo te` | Run tests for examples |

We're using `cargo-nextest` for testing, the commands above are described in [.cargo/config.toml](.cargo/config.toml).

## Special Thanks

- [MegaETH](https://github.com/megaeth-labs) for the funding and trust!
- [revm](https://github.com/bluealloy/revm) for the EVM in rust!

## LICENSE

GPL-3.0-only

[book]: https://zink-lang.org/
[telegram-badge]: https://img.shields.io/endpoint?label=chat&style=flat&url=https%3A%2F%2Fmogyo.ro%2Fquart-apis%2Ftgmembercount%3Fchat_id%3Dzinklang
[telegram-group]: https://t.me/zinklang
[version-badge]: https://img.shields.io/crates/v/zinkc
[version-link]: https://docs.rs/zinkc
[ci-badge]: https://img.shields.io/github/actions/workflow/status/clearloop/zink/main.yml
[ci-link]: https://github.com/clearloop/zink/actions/workflows/main.yml
[rustc-codegen]: https://doc.rust-lang.org/rustc/codegen-options/index.html
[wasm-opt]: https://github.com/WebAssembly/binaryen#binaryen-optimizations
