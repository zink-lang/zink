use anyhow::Result;
use zint::{Bytes32, EVM};

mod common;

#[ignore]
#[test]
fn fibonacci() -> Result<()> {
    let bytecode = common::load("recursion", "fibonacci")?;

    // x = 0
    let info = EVM::run(&bytecode, &0.to_bytes32());
    assert_eq!(0.to_bytes32().to_vec(), info.ret);

    // // x = 1
    // let info = EVM::run(&bytecode, &1.to_bytes32());
    // assert_eq!(1.to_bytes32().to_vec(), info.ret);
    //
    // // x = 3
    // let info = EVM::run(&bytecode, &3.to_bytes32());
    // assert_eq!(3.to_bytes32().to_vec(), info.ret);

    Ok(())
}
