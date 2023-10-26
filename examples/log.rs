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
