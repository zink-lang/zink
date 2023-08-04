# Command Line Tool

The zink toolchain are gathered in [zinkup][zinkup]

You can install all of the components directly with:

```bash
cargo install zinkup
```

For installing only specified binaries:

```bash
cargo install zinkup --features elko,zinkc
```

Available binaries:

| Name    | Description             |
| ------- | ----------------------- |
| `elko`  | Zink\'s package manager |
| `zinkc` | The zink compiler       |

[zinkup]: https://crates.io/crates/zinkup
