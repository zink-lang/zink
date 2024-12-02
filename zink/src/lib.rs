//! Zink library for developing smart contracts for blockchains.

#![no_std]

#[cfg(not(target_family = "wasm"))]
extern crate alloc;

mod asm;
mod event;
pub mod ffi;
pub mod primitives;
pub mod storage;
pub use self::{asm::Asm, event::Event};
pub use storage::{DoubleKeyMapping, Mapping, Storage, TransientStorage};
pub use zink_codegen::{external, revert, storage, Event};
pub use zink_codegen::transient_storage;

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

/// Convert bytes to ls bytes
#[cfg(not(target_family = "wasm"))]
pub fn to_bytes32(src: &[u8]) -> [u8; 32] {
    use alloc::vec::Vec;
    let mut bytes = [0u8; 32];
    let ls_bytes = {
        src.iter()
            .cloned()
            .rev()
            .skip_while(|b| *b == 0)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
    };

    bytes[(32 - ls_bytes.len())..].copy_from_slice(&ls_bytes);
    bytes
}

// Panic hook implementation
#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
