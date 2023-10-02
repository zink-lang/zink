//! Tests for instruction `select`.

use anyhow::Result;
use zint::{Bytes32, EVM};

mod common;

#[test]
fn log0() -> Result<()> {
    let bytecode = common::load("log", "log0")?;

    // returns the bigger number.
    let info = EVM::run(&bytecode, &[]);
    assert_eq!(info.logs[0].data.to_vec(), b"Ping".to_vec().to_bytes32());
    Ok(())
}

#[test]
fn log1() -> Result<()> {
    let bytecode = common::load("log", "log1")?;

    // returns the bigger number.
    let info = EVM::run(&bytecode, &[]);
    assert_eq!(info.logs[0].data.to_vec(), b"Ping".to_vec().to_bytes32());
    assert_eq!(
        info.logs[0].topics[0].to_vec(),
        b"pong".to_vec().to_bytes32()
    );
    Ok(())
}

#[test]
fn log2() -> Result<()> {
    let bytecode = common::load("log", "log2")?;

    // returns the bigger number.
    let info = EVM::run(&bytecode, &[]);
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
    let bytecode = common::load("log", "log3")?;

    // returns the bigger number.
    let info = EVM::run(&bytecode, &[]);
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
    let bytecode = common::load("log", "log4")?;

    // returns the bigger number.
    let info = EVM::run(&bytecode, &[]);
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

// #[test]
// fn vyper() -> Result<()> {
//     let bytecode = "6003361161000c57610127565b5f3560e01c3461012b5763ece866b98118610125577fcf8d08d4ab9d61004e3c20715af5b44c3badc3d3f41ddccbedbef447355ebff460408060c05260046040527f50696e670000000000000000000000000000000000000000000000000000000060605260408160c00181516020830160208301815181525050808252508051806020830101601f825f03163682375050601f19601f8251602001011690509050810190508060e05260046080527f706f6e670000000000000000000000000000000000000000000000000000000060a05260808160c00181516020830160208301815181525050808252508051806020830101601f825f03163682375050601f19601f82516020010116905090508101905060c0a1005b505b5f5ffd5b5f80fda165767970657283000309000b";
//     let info = EVM::run(
//         &hex::decode(bytecode).unwrap(),
//         &hex::decode("ece866b9").unwrap(),
//     );
//     println!("{:?}", info);
//
//     Ok(())
// }
