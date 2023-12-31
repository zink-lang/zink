//! Tests for instruction `select`.

use anyhow::Result;
use filetests::Test;
use zint::{Bytes32, Contract};

#[test]
fn params() -> Result<()> {
    let mut contract = Contract::from(Test::SELECT_PARAMS).pure().compile()?;
    let info = contract.execute([1, 2])?;
    assert_eq!(info.ret, [2.to_bytes32()].concat());

    let info = contract.execute([2, 1])?;
    assert_eq!(info.ret, [2.to_bytes32()].concat());
    Ok(())
}
