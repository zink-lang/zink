//! if-else tests for the zink compiler.
use anyhow::Result;
use filetests::Test;
use zint::{Bytes32, Contract, InstructionResult};

#[test]
fn if_then() -> Result<()> {
    let mut contract = Contract::new(Test::IF_BASIC)
        .without_dispatcher()
        .compile()?;

    // Skip the condition.
    let input = [0; 32];
    let info = contract.execute(&[input])?;
    assert_eq!(info.ret, input);

    // Enter the if branch.
    let input = 1.to_bytes32();
    let info = contract.execute(&[input])?;
    assert_eq!(info.ret, input);

    Ok(())
}

#[test]
fn singular() -> Result<()> {
    let mut contract = Contract::new(Test::IF_SINGULAR)
        .without_dispatcher()
        .compile()?;

    // test if
    //
    // Enter if block if 1
    let info = contract.execute(&[1])?;
    assert_eq!(info.instr, InstructionResult::Return);
    assert_eq!(info.ret, 7.to_bytes32());

    // test else
    let info = contract.execute(&[0])?;
    assert_eq!(info.instr, InstructionResult::Return);
    assert_eq!(info.ret, 8.to_bytes32());

    Ok(())
}
