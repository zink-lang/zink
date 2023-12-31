//! if-else tests for the zink compiler.
#![cfg(test)]

use anyhow::Result;
use filetests::Test;
use zint::{Bytes32, Contract};

#[test]
fn dummy() -> Result<()> {
    let mut contract = Contract::from(Test::CALL_DUMMY).pure().compile()?;
    let info = contract.execute::<()>([])?;

    assert!(info.ret.is_empty());
    Ok(())
}

#[test]
fn params() -> Result<()> {
    let mut contract = Contract::from(Test::CALL_PARAMS).pure().compile()?;
    let info = contract.execute([1, 2])?;

    assert_eq!(info.ret, 3.to_bytes32());
    Ok(())
}

#[test]
fn as_if() -> Result<()> {
    let mut contract = Contract::from(Test::CALL_AS_IF).pure().compile()?;

    let info = contract.execute([0])?;
    assert_eq!(info.ret, 0.to_bytes32());

    let info = contract.execute([1])?;
    assert_eq!(info.ret, 1.to_bytes32());

    let info = contract.execute([2])?;
    assert_eq!(info.ret, 41.to_bytes32());

    let info = contract.execute([3])?;
    assert_eq!(info.ret, 42.to_bytes32());
    Ok(())
}
