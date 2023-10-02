//! Addition example.
#![no_std]

// for the panic handler.
#[cfg(not(test))]
extern crate zink;

use zink::Event;

/// A `Ping` event.
///
/// TODO: generate this with proc-macro.
struct Ping;

/// TODO: generate this with proc-macro.
impl Event for Ping {
    const NAME: &'static [u8] = b"Ping";
}

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
