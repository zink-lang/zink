//! Zink library for developing smart contracts for blockchains.

#![no_std]

mod asm;
mod event;
pub mod ffi;
mod storage;

pub use self::{asm::Asm, event::Event, storage::Storage};
pub use zink_codegen::{constructor, external, storage, Event};

/// EVM address in rust.
pub type Address = [u8; 20];

// Panic hook implementation
#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
