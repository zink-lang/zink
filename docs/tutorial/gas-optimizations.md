# Gas Optimizations

### 1. Do not use `!=`

Since EVM opcodes don't have `NE`, `!=` means `Opcode::EQ` + `Opcode::ISZERO`, 
if it is possible to use `==` instead, never try `!=`.S


| OPCODE | NAME   | MINIMUM GAS | STACK INPUT | STACK OUTPUT | DESCRIPTION         |
|--------|--------|-------------|-------------|--------------|---------------------|
| 0x14   | EQ     | 3           | `a` `b`     | `a == b`     | Equality comparison |
| 0x15   | ISZERO | 3           | `a`         | `a == 0`     | Simple not operator |

```
// `==` costs 3 in total
EQ  // 3

// `!=` costs in total
EQ  // 3
NOT // 2
```
