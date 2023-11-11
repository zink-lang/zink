//! Zink library for developing smart contracts for blockchains.

#![no_std]

pub mod ffi;
mod traits;

pub use traits::{Event, Storage};
pub use zink_codegen::{constructor, external, storage, Event};

// Panic hook implementation
#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
