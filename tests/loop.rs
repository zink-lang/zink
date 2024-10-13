//! loop tests

use anyhow::Result;
use filetests::Test;
use zint::{Bytes32, Contract, HaltReason, OutOfGasError};

#[test]
fn singular() -> Result<()> {
    let mut contract = Contract::from(Test::LOOP_SINGULAR).pure().compile()?;
    let info = contract.execute::<()>([])?;

    assert_eq!(info.ret, 7.to_bytes32());

    Ok(())
}

#[test]
fn as_br_if() -> Result<()> {
    let mut contract = Contract::from(Test::LOOP_AS_BR_IF).pure().compile()?;
    let info = contract.execute([0])?;
    assert_eq!(info.halt, Some(HaltReason::OutOfGas(OutOfGasError::Basic)));

    let info = contract.execute([1])?;
    assert_eq!(info.ret, 7.to_bytes32());

    Ok(())
}
