//! Zink library for developing smart contracts for blockchains.

#![no_std]

extern crate alloc;

mod event;
pub mod ffi;

pub use self::event::Event;
pub use zink_codegen::Storage;

// Panic hook implementation
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
