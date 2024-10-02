//! Zink FFI.

pub mod asm;
pub mod evm;

#[link(wasm_import_module = "zinkc")]
#[allow(improper_ctypes)]
extern "C" {
    /// Emit ABI to host state.
    pub fn emit_abi(ptr: u32, len: u32);

    /// Load parameter
    pub fn load(ptr: u32, len: usize) -> u32;
}
