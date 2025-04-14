//! Zink FFI.

pub mod bytes;
pub mod evm;
pub mod ext;

#[link(wasm_import_module = "zinkc")]
#[allow(improper_ctypes)]
extern "C" {
    /// Emit ABI to host state.
    pub fn emit_abi(ptr: u32, len: u32);

    /// Set up a label for reserving 32 bytes in memory
    pub fn label_reserve_mem_32();

    /// Set up a label for reserving 64 bytes in memory
    pub fn label_reserve_mem_64();
}
