//! br_if tests for the zink compiler.
use anyhow::Result;
use filetests::Test;
use zint::{Contract, InstructionResult};

#[test]
fn as_block_last() -> Result<()> {
    let mut contract = Contract::new(Test::BR_IF_AS_BLOCK_LAST)
        .without_dispatcher()
        .compile()?;

    let info = contract.execute(&[0])?;
    assert_eq!(info.instr, InstructionResult::Return);
    assert!(info.ret.is_empty());

    let info = contract.execute(&[42])?;
    assert_eq!(info.instr, InstructionResult::OutOfGas);
    assert!(info.ret.is_empty());

    Ok(())
}
