//! Zink library for developing smart contracts for blockchains.

#![no_std]

pub mod ffi;
mod storage;
mod traits;

pub use self::{storage::Storage, traits::Event};
pub use zink_codegen::{constructor, external, storage, Event};

// Panic hook implementation
#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
