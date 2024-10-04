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

/// Generate a keccak hash of the input (sha3)
#[cfg(not(target_family = "wasm"))]
pub fn keccak256(input: &[u8]) -> [u8; 32] {
    use tiny_keccak::{Hasher, Keccak};
    let mut hasher = Keccak::v256();
    let mut output = [0; 32];
    hasher.update(input);
    hasher.finalize(&mut output);
    output
}

// Panic hook implementation
#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
