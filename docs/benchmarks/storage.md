# Storage

Result for a simple storage IO.

Have to say `Vyper` is super on this even it contains the logic of function
selector!

### Gas Cost

| Zink  | Vyper@0.3.9 | Solidity@0.8.21 |
| ----- | ----------- | --------------- |
| 22237 | 22345       | 27738           |

The gas costs here are measured by `transaction cost` + `execution cost`,
for example, the transaction of this function in solidity is `24120`, and
`2916` for the execution cost, `27738` in total.

Since revm doesn't support this and we haven't implemented the constructor
yet, we don't have the separated costs for zink for now ))

Issues: [zink-lang/zink#102][102], [bluealloy/revm#619][619]

### Runtime Code

| zink | vyper | solidity |
| ---- | ----- | -------- |
| 42   | 204   | 724      |

## `zink`

```rust
/// TODO: generate this storage interface with proc macro.
struct Counter;

impl Counter {
    fn get() -> i64 {
        unsafe { sload(0) }
    }

    fn set(value: i64) {
        unsafe { sstore(0, value) }
    }
}

/// Set value to the storage and get it.
#[no_mangle]
pub unsafe extern "C" fn set_and_get(value: i64) -> i64 {
    Counter::set(value);
    Counter::get()
}
```

```
6000600035589155600058905460005260206000f3
```

## `vyper`

```python
n: public(int256)

@external
def sg(_n: int256) -> int256:
  self.n = _n
  return self.n
```

```
6003361161000c57610051565b5f3560e01c3461005557632e52d606811861002c575f5460405260206040f35b63da48b556811861004f5760243610610055576004355f555f5460405260206040f35b505b5f5ffd5b5f80fda165767970657283000309000b
```

## `solidity`

```sol
pragma solidity >=0.7.0 <0.9.0;

contract Storage {
  int public number;

  function sg(int n) public returns (int) {
    number = n;
    return number;
  }
}
```

```
608060405234801561001057600080fd5b50600436106100365760003560e01c80638381f58a1461003b578063da48b55614610059575b600080fd5b610043610089565b60405161005091906100bb565b60405180910390f35b610073600480360381019061006e9190610107565b61008f565b60405161008091906100bb565b60405180910390f35b60005481565b6000816000819055506000549050919050565b6000819050919050565b6100b5816100a2565b82525050565b60006020820190506100d060008301846100ac565b92915050565b600080fd5b6100e4816100a2565b81146100ef57600080fd5b50565b600081359050610101816100db565b92915050565b60006020828403121561011d5761011c6100d6565b5b600061012b848285016100f2565b9150509291505056fea264697066735822122052e14a565911c984f75788fb539e44d7692065628b2042665fc4abfc95e680d264736f6c63430008120033
```

[102]: https://github.com/zink-lang/zink/issues/102
[104]: https://github.com/zink-lang/zink/issues/104
[619]: https://github.com/bluealloy/revm/issues/619
