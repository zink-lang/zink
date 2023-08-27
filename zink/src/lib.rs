//! Zink library for developing smart contracts for blockchains.

#![no_std]

mod event;
mod ffi;

pub use self::{
    event::Event,
    ffi::{evm, memory},
};

// Panic hook implementation
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
