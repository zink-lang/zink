//! Stack stability tests for the Zink compiler.
#![cfg(test)]

use anyhow::Result;
use filetests::Test;
use zint::{Bytes32, Contract};

#[test]
fn if_else_stack() -> Result<()> {
    let mut contract = Contract::from(Test::BR_IF_AS_IF_ELSE).pure().compile()?;
    tracing::debug!("Bytecode: {:?}", hex::encode(contract.bytecode().unwrap()));
    let info = contract.execute(&[1])?;
    assert_eq!(info.ret, 7.to_bytes32());
    let info = contract.execute(&[0])?;
    assert_eq!(info.ret, 8.to_bytes32());
    Ok(())
}

#[test]
fn call_no_return() -> Result<()> {
    let mut contract = Contract::from(Test::CALL_DUMMY).pure().compile()?;
    let info = contract.execute::<()>([])?;
    assert!(info.ret.is_empty(), "Dummy call should return nothing");
    assert!(
        info.halt.is_none(),
        "Execution should not halt unexpectedly"
    );
    Ok(())
}

#[test]
fn call_with_params() -> Result<()> {
    let mut contract = Contract::from(Test::CALL_PARAMS).pure().compile()?;
    let info = contract.execute([1, 2])?;
    assert_eq!(info.ret, 3.to_bytes32(), "Call should return sum of params");
    assert!(
        info.halt.is_none(),
        "Execution should not halt unexpectedly"
    );
    Ok(())
}

#[test]
fn loop_stack() -> Result<()> {
    let mut contract = Contract::from(Test::LOOP_SINGULAR).pure().compile()?;
    let info = contract.execute::<()>([])?;
    assert_eq!(info.ret, 7.to_bytes32(), "Loop should return 7");
    Ok(())
}

#[test]
fn conditional_return() -> Result<()> {
    let mut contract = Contract::from(Test::STACK_CONDITIONAL).pure().compile()?;
    let info = contract.execute(&[0])?;
    assert_eq!(info.ret, 1.to_bytes32(), "If branch should return 1");
    let info = contract.execute(&[1])?;
    assert_eq!(info.ret, 1.to_bytes32(), "Else branch should return 1");
    Ok(())
}

#[test]
fn dispatcher_stack() -> Result<()> {
    tracing::debug!(
        "STACK_DISPATCH bytes: {:?}",
        hex::encode(&Test::STACK_DISPATCH)
    );
    let mut contract = Contract::from(Test::STACK_DISPATCH).pure().compile()?;
    tracing::debug!(
        "Compiled bytecode: {:?}",
        hex::encode(contract.bytecode().unwrap())
    );
    let info = contract.execute(&[1])?;
    tracing::debug!("Return value for input 1: {:?}", info.ret);
    assert_eq!(info.ret, 2.to_bytes32(), "func1 should return input + 1");
    let info = contract.execute(&[0])?;
    tracing::debug!("Return value for input 0: {:?}", info.ret);
    assert_eq!(info.ret, 2.to_bytes32(), "func2 should return input + 2");
    Ok(())
}
