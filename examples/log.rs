//! Addition example.
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", no_main)]

extern crate zink;

use zink::Event;

/// A `Ping` event.
#[derive(Event)]
struct Ping;

#[no_mangle]
pub extern "C" fn log0() {
    Ping.log0();
}

#[no_mangle]
pub extern "C" fn log1() {
    Ping.log1(b"pong");
}

#[no_mangle]
pub extern "C" fn log2() {
    Ping.log2(b"pong", b"ping");
}

#[no_mangle]
pub extern "C" fn log3() {
    Ping.log3(b"pong", b"ping", b"pong");
}

#[no_mangle]
pub extern "C" fn log4() {
    Ping.log4(b"pong", b"ping", b"pong", b"pong");
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
