//! EVM FFI.

use crate::primitives::{Address, Bytes32};

#[link(wasm_import_module = "evm")]
#[allow(improper_ctypes)]
extern "C" {
    /// Push 1 byte to the stack.
    pub fn push0();

    /// Push 1 byte to the stack.
    pub fn push1(val: i32);
 
    /// Push 2 bytes to the stack.
    pub fn push2(val: i32);

    /// Push 3 bytes to the stack.
    pub fn push3(val: i32);

    /// Push 4 bytes to the stack.
    pub fn push4(val: i32);

    /// Push 5 bytes to the stack.
    pub fn push5(val: i32);

    /// Push 6 bytes to the stack.
    pub fn push6(val: i32);

    /// Push 7 bytes to the stack.
    pub fn push7(val: i32);

    /// Push 8 bytes to the stack.
    pub fn push8(val: i32);

    /// Push 9 bytes to the stack.
    pub fn push9(val: i32);

    /// Push 10 bytes to the stack.
    pub fn push10(val: i32);

    /// Push 11 bytes to the stack.
    pub fn push11(val: i32);

    /// Push 12 bytes to the stack.
    pub fn push12(val: i32);

    /// Push 13 bytes to the stack.
    pub fn push13(val: i32);

    /// Push 14 bytes to the stack.
    pub fn push14(val: i32);

    /// Push 15 bytes to the stack.
    pub fn push15(val: i32);

    /// Push 16 bytes to the stack.
    pub fn push16(val: i32);

    /// Push 17 bytes to the stack.
    pub fn push17(val: i32);

    /// Push 18 bytes to the stack.
    pub fn push18(val: i32);

    /// Push 19 bytes to the stack.
    pub fn push19(val: i32);

    /// Push 20 bytes to the stack.
    pub fn push20(val: i32);

    /// Push 21 bytes to the stack.
    pub fn push21(val: i32);

    /// Push 22 bytes to the stack.
    pub fn push22(val: i32);

    /// Push 23 bytes to the stack.
    pub fn push23(val: i32);

    /// Push 24 bytes to the stack.
    pub fn push24(val: i32);

    /// Push 25 bytes to the stack.
    pub fn push25(val: i32);

    /// Push 26 bytes to the stack.
    pub fn push26(val: i32);

    /// Push 27 bytes to the stack.
    pub fn push27(val: i32);

    /// Push 28 bytes to the stack.
    pub fn push28(val: i32);

    /// Push 29 bytes to the stack.
    pub fn push29(val: i32);

    /// Push 30 bytes to the stack.
    pub fn push30(val: i32);

    /// Push 31 bytes to the stack.
    pub fn push31(val: i32);

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

    /// Append log record with one topic
    pub fn log1(topic1: Bytes32, name: &'static [u8]);

    /// Append log record with two topics
    pub fn log2(topic1: Bytes32, topic2: Bytes32, name: &'static [u8]);

    /// Append log record with three topics
    pub fn log3(topic1: Bytes32, topic2: Bytes32, topic3: Bytes32, name: &'static [u8]);

    /// Append log record with four topics
    pub fn log4(
        topic1: Bytes32,
        topic2: Bytes32,
        topic3: Bytes32,
        topic4: Bytes32,
        name: &'static [u8],
    );
}
