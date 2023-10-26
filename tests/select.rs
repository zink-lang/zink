//! Tests for instruction `select`.

use anyhow::Result;
use zinkc_filetests::Test;
use zint::{Bytes32, Contract};

#[test]
fn params() -> Result<()> {
    let mut contract = Contract::new(Test::SELECT_PARAMS)
        .without_dispatcher()
        .compile()?;
    let info = contract.execute([1, 2])?;
    assert_eq!(info.ret, [2.to_bytes32()].concat());

    let info = contract.execute([2, 1])?;
    assert_eq!(info.ret, [2.to_bytes32()].concat());
    Ok(())
}
