//! loop tests

use anyhow::Result;
use zint::{Bytes32, Contract, InstructionResult};

#[test]
fn singular() -> Result<()> {
    let mut contract = Contract::new(filetests::LOOP_SINGULAR)
        .without_dispatcher()
        .compile()?;
    let info = contract.execute::<()>([])?;

    assert_eq!(info.ret, 7.to_bytes32());

    Ok(())
}

#[test]
fn as_br_if() -> Result<()> {
    let mut contract = Contract::new(filetests::LOOP_AS_BR_IF)
        .without_dispatcher()
        .compile()?;
    let info = contract.execute([0])?;
    assert_eq!(info.instr, InstructionResult::OutOfGas);

    let info = contract.execute([1])?;
    assert_eq!(info.instr, InstructionResult::Return);
    assert_eq!(info.ret, 7.to_bytes32());

    Ok(())
}
