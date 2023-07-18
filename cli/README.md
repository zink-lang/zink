# `zinkup`

```
cargo install zinkup
```

## Binary `elko`

``` shell
elko
Zink's package manager

Usage: elko [OPTIONS] <COMMAND>

Commands:
  new    Create a new zink project
  build  Build zink project to EVM bytecode
  help   Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Verbose mode (-v, -vv, -vvv, etc.)
  -h, --help        Print help
  -V, --version     Print version
```


## Binary `zinkc`

```
Zink Compiler

Usage: zinkc [OPTIONS] <INPUT>

Arguments:
  <INPUT>  The path of the wasm file

Options:
  -o, --output <OUTPUT>  Write output to <filename>
  -v, --verbose...       Verbose mode (-v, -vv, -vvv, etc.)
  -h, --help             Print help
  -V, --version          Print version
```
