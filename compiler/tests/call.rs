//! if-else tests for the zink compiler.
#![cfg(test)]

use anyhow::Result;
use zinkc::Compiler;
use zint::EVM;

mod common;

#[test]
fn params() -> Result<()> {
    let wasm = common::load("call", "params")?;
    let bytecode = Compiler::default().compile(&wasm)?;

    tracing::trace!("{:x?}", bytecode);

    #[rustfmt::skip]
    let expected = [
        0x60, 0x0, 0x35,  // PUSH1 0x00 CALLDATALOAD
        0x60, 0x20, 0x35, // PUSH1 0x20 CALLDATALOAD
        0x60, 0x12,       // PUSH1 0x13
        0x56,             // JUMP
        0x5b,             // JUMPDEST
        0x60, 0x0, 0x52,  // PUSH1 0x00 MSTORE
        0x60, 0x20,       // PUSH1 0x20
        0x60, 0x0,        // PUSH1 0x00
        0xf3,             // RETURN
        0x5b,             // JUMPDEST
        0x1,              // ADD
        0x60, 0x09,       // PUSH1 0x09
        0x56,             // JUMP
    ];

    // Skip the condition.
    let input = [vec![0; 31], vec![1; 1], vec![0; 31], vec![2; 1]].concat();
    let (ret, instr) = EVM::run(&expected, &input);
    tracing::trace!("{:?}", instr);

    assert_eq!(ret, [vec![0; 31], vec![3; 1]].concat());

    // // Enter the if branch.
    // let mut input = vec![0; 31];
    // input.push(1);
    // let (ret, _) = EVM::run(&bytecode, &input);
    // assert_eq!(ret, input);

    Ok(())
}
