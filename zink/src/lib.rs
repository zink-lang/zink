//! Zink library for developing smart contracts for blockchains.

#![no_std]

mod asm;
mod event;
pub mod ffi;
pub mod primitives;
pub mod storage;

pub use self::{asm::Asm, event::Event};
pub use storage::{DoubleKeyMapping, Mapping, Storage};
pub use zink_codegen::{external, storage, Event};

// Panic hook implementation
#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
