//! Zink library for developing smart contracts for blockchains.

#![no_std]

mod asm;
mod event;
pub mod ffi;
mod storage;

pub use self::{asm::Asm, event::Event, storage::Storage};
pub use zink_codegen::{external, storage, Event};

// Panic hook implementation
#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
