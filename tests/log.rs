//! Tests for instruction `select`.

use anyhow::Result;
use zinkc_filetests::Test;
use zint::{Bytes32, Contract};

#[test]
fn log0() -> Result<()> {
    let mut contract = Contract::new(Test::LOG_LOG0)
        .without_dispatcher()
        .compile()?;

    // returns the bigger number.
    let info = contract.execute::<()>([])?;
    assert_eq!(info.logs[0].data.to_vec(), b"Ping".to_vec().to_bytes32());
    Ok(())
}

#[test]
fn log1() -> Result<()> {
    let mut contract = Contract::new(Test::LOG_LOG1)
        .without_dispatcher()
        .compile()?;

    let info = contract.execute::<()>([])?;
    assert_eq!(info.logs[0].data.to_vec(), b"Ping".to_vec().to_bytes32());
    assert_eq!(
        info.logs[0].topics[0].to_vec(),
        b"pong".to_vec().to_bytes32()
    );
    Ok(())
}

#[test]
fn log2() -> Result<()> {
    let mut contract = Contract::new(Test::LOG_LOG2)
        .without_dispatcher()
        .compile()?;
    let info = contract.execute::<()>([])?;

    assert_eq!(info.logs[0].data.to_vec(), b"Ping".to_vec().to_bytes32());
    assert_eq!(
        info.logs[0].topics[0].to_vec(),
        b"pong".to_vec().to_bytes32()
    );
    assert_eq!(
        info.logs[0].topics[1].to_vec(),
        b"ping".to_vec().to_bytes32()
    );
    Ok(())
}

#[test]
fn log3() -> Result<()> {
    let mut contract = Contract::new(Test::LOG_LOG3)
        .without_dispatcher()
        .compile()?;
    let info = contract.execute::<()>([])?;

    assert_eq!(info.logs[0].data.to_vec(), b"Ping".to_vec().to_bytes32());
    assert_eq!(
        info.logs[0].topics[0].to_vec(),
        b"pong".to_vec().to_bytes32()
    );
    assert_eq!(
        info.logs[0].topics[1].to_vec(),
        b"ping".to_vec().to_bytes32()
    );
    assert_eq!(
        info.logs[0].topics[2].to_vec(),
        b"pong".to_vec().to_bytes32()
    );
    Ok(())
}

#[test]
fn log4() -> Result<()> {
    let mut contract = Contract::new(Test::LOG_LOG4)
        .without_dispatcher()
        .compile()?;
    let info = contract.execute::<()>([])?;

    assert_eq!(info.logs[0].data.to_vec(), b"Ping".to_vec().to_bytes32());
    assert_eq!(
        info.logs[0].topics[0].to_vec(),
        b"pong".to_vec().to_bytes32()
    );
    assert_eq!(
        info.logs[0].topics[1].to_vec(),
        b"ping".to_vec().to_bytes32()
    );
    assert_eq!(
        info.logs[0].topics[2].to_vec(),
        b"pong".to_vec().to_bytes32()
    );
    assert_eq!(
        info.logs[0].topics[3].to_vec(),
        b"pong".to_vec().to_bytes32()
    );
    Ok(())
}
