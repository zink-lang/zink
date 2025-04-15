//! Stack stability tests for the Zink compiler.
#![cfg(test)]

use anyhow::Result;
use filetests::Test;
use zint::{Bytes32, Contract};

#[test]
fn dispatcher_stack() -> Result<()> {
    let mut contract = Contract::from(Test::STACK_DISPATCHER).pure().compile()?;

    // Test input 1: Should call func1 (input + 1), expect return value 2
    let info = contract.execute(&[1.to_bytes32()])?;
    assert_eq!(info.ret, 2.to_bytes32());

    // Test input 0: Should call func2 (input + 2), expect return value 2
    let info = contract.execute(&[0.to_bytes32()])?;
    assert_eq!(info.ret, 2.to_bytes32());

    Ok(())
}