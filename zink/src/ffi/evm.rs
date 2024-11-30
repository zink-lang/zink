//! EVM FFI.

use crate::primitives::Address;

#[link(wasm_import_module = "evm")]
#[allow(improper_ctypes)]
extern "C" {
    /// Push 1 byte to the stack.
    pub fn push0();

    /// Push 1 byte to the stack.
    pub fn push1(val: i32);

    // ... existing push functions ...
    
    /// Push 32 bytes to the stack.
    pub fn push32();

    /// Store a value in the storage
    pub fn sstore();

    /// Load a value from the storage
    pub fn sload();

    /// Store a value in the transient storage
    pub fn tstore();

    /// Load a value from the transient storage
    pub fn tload();

    /// Save word to memory
    pub fn mstore();

    /// Save byte to memory
    pub fn mstore8();

    /// Load word from memory
    pub fn mload();

    /// Copy memory to memory
    pub fn mcopy();

    /// Compute Keccak-256 hash
    pub fn keccak256();

    /// Get the current message sender
    pub fn caller() -> Address;

    /// Get the current blob hash at index
    pub fn blobhash();

    /// Get the current blob base fee
    pub fn blobbasefee();

    /// Append log record with no topics
    pub fn log0(name: &'static [u8]);

    /// Append log record with one topics
    pub fn log1(name: &'static [u8], topic1: &'static [u8]);

    /// Append log record with two topics
    pub fn log2(name: &'static [u8], topic1: &'static [u8], topic2: &'static [u8]);

    /// Append log record with three topics
    pub fn log3(
        name: &'static [u8],
        topic1: &'static [u8],
        topic2: &'static [u8],
        topic3: &'static [u8],
    );

    /// Append log record with four topics
    pub fn log4(
        name: &'static [u8],
        topic1: &'static [u8],
        topic2: &'static [u8],
        topic3: &'static [u8],
        topic4: &'static [u8],
    );
}