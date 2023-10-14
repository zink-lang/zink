//! Zink library for developing smart contracts for blockchains.

#![no_std]

pub mod ffi;
mod traits;

pub use traits::{Event, Storage};
pub use zink_codegen::{external, storage, Event};

// Panic hook implementation
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
