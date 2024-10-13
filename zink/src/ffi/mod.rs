//! Zink FFI.

use crate::primitives::Address;

pub mod asm;
pub mod evm;

#[link(wasm_import_module = "zinkc")]
#[allow(improper_ctypes)]
extern "C" {
    /// Emit ABI to host state.
    pub fn emit_abi(ptr: u32, len: u32);

    /// Equal operation for addresses
    pub fn address_eq(this: Address, other: Address) -> bool;

    /// Set up a label for reserving 32 bytes in memory
    pub fn label_reserve_mem_32();

    /// Set up a label for reserving 64 bytes in memory
    pub fn label_reserve_mem_64();
}
