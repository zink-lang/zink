//! Zink FFI.

use crate::primitives::{Address, U256};

pub mod asm;
pub mod bytes;
pub mod evm;

#[link(wasm_import_module = "zinkc")]
#[allow(improper_ctypes)]
extern "C" {
    /// Emit ABI to host state.
    pub fn emit_abi(ptr: u32, len: u32);

    /// Equal operation for addresses
    pub fn address_eq(this: Address, other: Address) -> bool;

    /// Equal operation for addresses
    pub fn u256_add(this: U256, other: U256) -> U256;

    /// Equal operation for addresses
    pub fn u256_sub(this: U256, other: U256) -> U256;

    /// Less than operation for addresses
    pub fn u256_lt(this: U256, other: U256) -> bool;

    /// Equal operation for addresses
    pub fn u256_eq(this: U256, other: U256) -> bool;

    /// Returns zero value
    pub fn u256_zero() -> U256;

    /// Equal operation for addresses
    pub fn u256_max() -> U256;

    /// Addmod operation for addresses
    pub fn u256_addmod(this: U256, other: U256, modulus: U256) -> U256;

    /// Equal operation for addresses
    pub fn u256_mulmod(this: U256, other: U256, modulus: U256) -> U256;

    /// Set up a label for reserving 32 bytes in memory
    pub fn label_reserve_mem_32();

    /// Set up a label for reserving 64 bytes in memory
    pub fn label_reserve_mem_64();
}
