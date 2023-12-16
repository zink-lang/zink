use anyhow::Result;
use filetests::Test;
use zint::{Bytes32, Contract};

#[test]
fn fibonacci() -> Result<()> {
    let mut contract = Contract::new(Test::RECURSION_FIBONACCI)
        .without_dispatcher()
        .compile()?;

    // x = 0
    let info = contract.execute([0])?;
    assert_eq!(0.to_bytes32().to_vec(), info.ret);

    // x = 1
    let info = contract.execute([1])?;
    assert_eq!(1.to_bytes32().to_vec(), info.ret);

    // x = 2
    let info = contract.execute([2])?;
    assert_eq!(1.to_bytes32().to_vec(), info.ret);

    // x = 3
    let info = contract.execute([3])?;
    assert_eq!(2.to_bytes32().to_vec(), info.ret);

    // x = 4
    let info = contract.execute([4])?;
    assert_eq!(3.to_bytes32().to_vec(), info.ret);

    // x = 5
    let info = contract.execute([5])?;
    assert_eq!(5.to_bytes32().to_vec(), info.ret);

    Ok(())
}
