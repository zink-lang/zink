# `elko` - Zink's package manager

## Installation

```bash
cargo install zinkup --features elko
```

## Usages

```bash
elko
Package manager of zink.

Usage: elko <COMMAND> [OPTIONS]

Commands:
  new    Create a new zink project
  build  Build zink project to EVM bytecode
  help   Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Verbose mode (-v, -vv, -vvv, etc.)
  -h, --help        Print help
  -V, --version     Print version 
  --example <EXAMPLE>  Specify an example template for `new` (e.g., "erc20", "addition"; defaults to "addition" if not set)
```

Examples:
  - Create a basic addition project:
    ```bash
    elko new myproj
    cd myproj
    elko build
    ```
  - Create an ERC20 token project:
    ```bash
    elko new mytoken --example erc20
    cd mytoken
    elko build
    ```

Notes:
  - Create a project directory with `elko new` before running `elko build`.
  - Subcommand options (like `--example`) apply after the command (e.g., `elko new mytoken --example erc20`).

## LICENSE

GPL-3.0
