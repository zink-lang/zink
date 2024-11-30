//! br_if tests for the zink compiler.
use anyhow::Result;
use filetests::Test;
use zint::{Contract, HaltReason};

#[test]
fn as_block_last() -> Result<()> {
    let mut contract = Contract::from(Test::BR_IF_AS_BLOCK_LAST).pure().compile()?;

    let info = contract.execute(&[0])?;
    assert!(info.halt.is_none());
    assert!(info.ret.is_empty());

    let info = contract.execute(&[42])?;
    assert!(matches!(info.halt, Some(HaltReason::OutOfGas(_))));
    assert!(info.ret.is_empty());

    Ok(())
}
