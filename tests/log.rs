//! Tests for instruction `select`.

use anyhow::Result;
use filetests::Test;
use zint::{Bytes32, Contract};

#[test]
fn log0() -> Result<()> {
    let mut contract = Contract::from(Test::LOG_LOG0).pure().compile()?;

    // returns the bigger number.
    let info = contract.execute::<()>([])?;
    assert_eq!(
        info.logs[0].data.data.to_vec(),
        b"Ping".to_vec().to_bytes32()
    );
    Ok(())
}

#[test]
fn log1() -> Result<()> {
    let mut contract = Contract::from(Test::LOG_LOG1).pure().compile()?;

    let info = contract.execute::<()>([])?;
    assert_eq!(
        info.logs[0].data.data.to_vec(),
        b"Ping".to_vec().to_bytes32()
    );
    assert_eq!(
        info.logs[0].topics()[0].to_vec(),
        b"pong".to_vec().to_bytes32()
    );
    Ok(())
}

#[test]
fn log2() -> Result<()> {
    let mut contract = Contract::from(Test::LOG_LOG2).pure().compile()?;
    let info = contract.execute::<()>([])?;

    assert_eq!(
        info.logs[0].data.data.to_vec(),
        b"Ping".to_vec().to_bytes32()
    );
    assert_eq!(
        info.logs[0].topics()[0].to_vec(),
        b"pong".to_vec().to_bytes32()
    );
    assert_eq!(
        info.logs[0].topics()[1].to_vec(),
        b"ping".to_vec().to_bytes32()
    );
    Ok(())
}

#[test]
fn log3() -> Result<()> {
    let mut contract = Contract::from(Test::LOG_LOG3).pure().compile()?;
    let info = contract.execute::<()>([])?;
    let topics = info.logs[0].topics();

    assert_eq!(
        info.logs[0].data.data.to_vec(),
        b"Ping".to_vec().to_bytes32()
    );
    assert_eq!(topics[0].to_vec(), b"pong".to_vec().to_bytes32());
    assert_eq!(topics[1].to_vec(), b"ping".to_vec().to_bytes32());
    assert_eq!(topics[2].to_vec(), b"pong".to_vec().to_bytes32());
    Ok(())
}

#[test]
fn log4() -> Result<()> {
    let mut contract = Contract::from(Test::LOG_LOG4).pure().compile()?;
    let info = contract.execute::<()>([])?;
    let topics = info.logs[0].topics();

    assert_eq!(
        info.logs[0].data.data.to_vec(),
        b"Ping".to_vec().to_bytes32()
    );
    assert_eq!(topics[0].to_vec(), b"pong".to_vec().to_bytes32());
    assert_eq!(topics[1].to_vec(), b"ping".to_vec().to_bytes32());
    assert_eq!(topics[2].to_vec(), b"pong".to_vec().to_bytes32());
    assert_eq!(topics[3].to_vec(), b"pong".to_vec().to_bytes32());
    Ok(())
}
