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
}
