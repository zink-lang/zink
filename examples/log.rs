//! Addition example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::Event;

/// A `Ping` event.
#[derive(Event)]
struct Ping;

#[zink::external]
pub fn log0() {
    Ping.log0();
}

#[zink::external]
pub fn log1() {
    Ping.log1(b"pong");
}

#[zink::external]
pub fn log2() {
    Ping.log2(b"pong", b"ping");
}

#[zink::external]
pub fn log3() {
    Ping.log3(b"pong", b"ping", b"pong");
}

#[zink::external]
pub fn log4() {
    Ping.log4(b"pong", b"ping", b"pong", b"pong");
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[test]
fn test() -> anyhow::Result<()> {
    use zint::{Bytes32, Contract};
    let mut contract = Contract::search("log")?.compile()?;

    let info = contract.execute(["log0()"])?;
    assert_eq!(
        info.logs[0].data.data.to_vec(),
        b"Ping".to_vec().to_bytes32()
    );

    let info = contract.execute(["log1()"])?;
    assert_eq!(
        info.logs[0].data.data.to_vec(),
        b"Ping".to_vec().to_bytes32()
    );
    assert_eq!(info.logs[0].topics(), vec![b"pong".to_vec().to_bytes32()]);

    let info = contract.execute(["log2()"])?;
    assert_eq!(
        info.logs[0].data.data.to_vec(),
        b"Ping".to_vec().to_bytes32()
    );
    assert_eq!(
        info.logs[0].topics(),
        vec![b"pong".to_vec().to_bytes32(), b"ping".to_vec().to_bytes32()]
    );

    let info = contract.execute(["log3()"])?;
    assert_eq!(
        info.logs[0].data.data.to_vec(),
        b"Ping".to_vec().to_bytes32()
    );
    assert_eq!(
        info.logs[0].topics(),
        vec![
            b"pong".to_vec().to_bytes32(),
            b"ping".to_vec().to_bytes32(),
            b"pong".to_vec().to_bytes32()
        ]
    );

    let info = contract.execute(["log4()"])?;
    assert_eq!(
        info.logs[0].data.data.to_vec(),
        b"Ping".to_vec().to_bytes32()
    );
    assert_eq!(
        info.logs[0].topics(),
        vec![
            b"pong".to_vec().to_bytes32(),
            b"ping".to_vec().to_bytes32(),
            b"pong".to_vec().to_bytes32(),
            b"pong".to_vec().to_bytes32()
        ]
    );

    Ok(())
}
