//! Zink FFI.

use crate::primitives::U256;

pub mod asm;
pub mod bytes;
pub mod evm;

#[link(wasm_import_module = "zinkc")]
#[allow(improper_ctypes)]
extern "C" {
    /// Emit ABI to host state.
    pub fn emit_abi(ptr: u32, len: u32);

    /// Addition operation for U256
    #[cfg(target_arch = "wasm32")]
    pub fn u256_add(this: U256, other: U256) -> U256;

    /// Subtraction operation for U256
    #[cfg(target_arch = "wasm32")]
    pub fn u256_sub(this: U256, other: U256) -> U256;

    /// Division operation for U256
    pub fn u256_div(this: U256, other: U256) -> U256;

    /// Less than operation for U256
    #[cfg(target_arch = "wasm32")]
    pub fn u256_lt(this: U256, other: U256) -> bool;

    /// Returns the maximum U256 value
    #[cfg(target_arch = "wasm32")]
    pub fn u256_max() -> U256;

    /// Addmod operation for U256
    #[cfg(target_arch = "wasm32")]
    pub fn u256_addmod(this: U256, other: U256, modulus: U256) -> U256;

    /// Mulmod operation for U256
    #[cfg(target_arch = "wasm32")]
    pub fn u256_mulmod(this: U256, other: U256, modulus: U256) -> U256;

    /// Set up a label for reserving 32 bytes in memory
    #[cfg(target_arch = "wasm32")]
    pub fn label_reserve_mem_32();

    /// Set up a label for reserving 64 bytes in memory
    #[cfg(target_arch = "wasm32")]
    pub fn label_reserve_mem_64();
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::U256;

    #[allow(dead_code)]
    pub fn emit_abi(_ptr: u32, _len: u32) {
        unimplemented!("emit_abi not implemented for native targets");
    }

    pub fn u256_add(_this: U256, _other: U256) -> U256 {
        unimplemented!("u256_add not implemented for native targets");
    }

    pub fn u256_sub(_this: U256, _other: U256) -> U256 {
        unimplemented!("u256_sub not implemented for native targets");
    }

    #[allow(dead_code)]
    pub fn u256_div(_this: U256, _other: U256) -> U256 {
        unimplemented!("u256_div not implemented for native targets");
    }

    pub fn u256_lt(_this: U256, _other: U256) -> bool {
        unimplemented!("u256_lt not implemented for native targets");
    }

    pub fn u256_max() -> U256 {
        unimplemented!("u256_max not implemented for native targets");
    }

    pub fn u256_addmod(_this: U256, _other: U256, _modulus: U256) -> U256 {
        unimplemented!("u256_addmod not implemented for native targets");
    }

    pub fn u256_mulmod(_this: U256, _other: U256, _modulus: U256) -> U256 {
        unimplemented!("u256_mulmod not implemented for native targets");
    }

    pub fn label_reserve_mem_32() {
        unimplemented!("label_reserve_mem_32 not implemented for native targets");
    }

    pub fn label_reserve_mem_64() {
        unimplemented!("label_reserve_mem_64 not implemented for native targets");
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;
