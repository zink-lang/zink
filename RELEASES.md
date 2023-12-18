## v0.1.9

### Changes

- Refactor conta with `toml_edit`
- Optional exports wasm-opt from zinkc
- Use full revm instead of ethers in zint

### FIXED

- Missing logic of adapt package alias in conta

## v0.1.8

### Added

- ABI output in zink compiler
- no_std solidity ABI

### Changes

- Solidity compatible ABI
- Refactor `zabi` a wrapper a `sol-abi`
- Conditional compilation for abi related crates
- Compile `zinkc` binary in crate `zinkc`
- Use generated ABI for the constructor tests
- Rename zinkc-filetests to filetests

## v0.1.7

### Added

- testing utils for deployment

## v0.1.6

### Added

- Contract constructor implementation.

## v0.1.5

### Added

- Function dispatcher
  - Crate `zabi`
  - Host function `emit_abi`
  - new `proc-macro` `zink::external`
  - `dispatcher` flag for `elko` and `zinkc`
  - Jump with offset in jump table
- `Contract` instance in `zint`
  - Built-in tests for all examples
  - filetests of the compiler

### Changed

- Map functions in codegen for different usages
- Move `zink` to the top level
- Move previous compiler tests to the top level
- Move examples out of crates
- The PC order of return and callee labels

### Fixed

- Add up original PC offset while shifting themselves in PC relocation
- clean stack on loading data from data section

---

## v0.1.4

### Added

- `proc-macro` for storage
- `proc-macro` for event logging
- Update documents for storage and events

### Fixed

- Publishing logic of `conta`

---

## v0.1.3

### Added

- Event logging APIs
- Examples for logging
- Data section parser in `codegen`
- Documents for event logging APIs
- Benchmarks for event logging APIs

---

## v0.1.2

### Added

- Storage related built-in functions
  - `sstore` and `sload`
- `impl_tests` for generating arithmetic tests
- Project logo
- rust-cache in CI
- Documents for storage APIs
- Benchmarks for event storage APIs

---

## v0.1.1

### Added

- Code section in `codegen`
- Instruction `select`
- Params test for `select`

---

## V0.1.0

The MVP of the zink project, provides various tools for developing
EVM contracts with `rust` and `WASM`.

### Binaries

| name    | description                                                 |
| ------- | ----------------------------------------------------------- |
| `elko`  | Zink's package manager, can create and build zink project.  |
| `zinkc` | The zink compiler, can compile simple wasm to EVM bytecode. |

For supporting nearly everything, plz keep tuned for `v0.3.0`.

### Components

| name      | description                                               |
| --------- | --------------------------------------------------------- |
| `zinkgen` | Zink code generator                                       |
| `zinkc`   | Zink compiler                                             |
| `zink`    | Rust library for developing program with zink             |
| `zint`    | Basic test utils including evm wrapper for testing usages |
| `zinkup`  | Zink toolchain installer                                  |

### Added

- provided basic functionalities in `v0.1.0` to verify thoughts, the final target
  of it is example `fibonaaci`, which means, everything used in the `fibonacci` example
  now works!
- `add`, `sub`, `mul` are available now, plus all comparison operand like `gt`, `lt`,
  `ge`, `le`, `bitwise` also have implementations **operators like `shr` require the
  order of the stack will have bugs\***.
- The compilation of locals currently works without any hardcode, ideally, we don't
  need to refactor it in the future!
- Same as locals, works without any hardcode, but some logic related to the jump table
  need to be refactored after introducing `selector`.
- `if`, `else`, `block`, `loop`, `br_if` now works without any hardcode, need to add
  `br_table`, `select`... to align wasm MVP in the future releases.~\*
