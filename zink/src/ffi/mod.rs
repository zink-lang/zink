//! Zink bindings for compilation
//!
//! Host functions in this module will not
//! being compiled to bytecode, but will be
//! called by the compiler.

pub mod evm;

// Zinkc interfaces
#[link(wasm_import_module = "zinkc")]
#[allow(improper_ctypes)]
extern "C" {
    /// Emit ABI to host state.
    pub fn emit_abi(ptr: u32, len: u32);
}
