//! Assembly FFI.
#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "asm")]
#[allow(improper_ctypes)]
extern "C" {
    /// Push a 8-bit signed integer to the stack.
    pub fn push_i8(val: i8);

    /// Push a 8-bit unsigned integer to the stack.
    pub fn push_u8(val: u8);

    /// Push a 16-bit signed integer to the stack.
    pub fn push_i16(val: i16);

    /// Push a 16-bit unsigned integer to the stack.
    pub fn push_u16(val: u16);

    /// Push a 32-bit signed integer to the stack.
    pub fn push_i32(val: i32);

    /// Push a 32-bit unsigned integer to the stack.
    pub fn push_u32(val: u32);

    /// Push a 64-bit signed integer to the stack.
    pub fn push_i64(val: i64);

    /// Push a 64-bit unsigned integer to the stack.
    pub fn push_u64(val: u64);

    /// Emit opcode ADDMOD
    pub fn addmod_i8(a: i8, b: i8, n: i8) -> i8;
    /// Emit opcode ADDMOD
    pub fn mulmod_i8(a: i8, b: i8, n: i8) -> i8;

    /// Emit opcode ADDMOD
    pub fn addmod_i16(a: i16, b: i16, n: i16) -> i16;
    /// Emit opcode ADDMOD
    pub fn mulmod_i16(a: i16, b: i16, n: i16) -> i16;

    /// Emit opcode ADDMOD
    pub fn addmod_i32(a: i32, b: i32, n: i32) -> i32;
    /// Emit opcode ADDMOD
    pub fn mulmod_i32(a: i32, b: i32, n: i32) -> i32;

    /// Emit opcode ADDMOD
    pub fn addmod_i64(a: i64, b: i64, n: i64) -> i64;
    /// Emit opcode ADDMOD
    pub fn mulmod_i64(a: i64, b: i64, n: i64) -> i64;

    /// Emit opcode ADDMOD
    pub fn addmod_u8(a: u8, b: u8, n: u8) -> u8;
    /// Emit opcode ADDMOD
    pub fn mulmod_u8(a: u8, b: u8, n: u8) -> u8;

    /// Emit opcode ADDMOD
    pub fn addmod_u16(a: u16, b: u16, n: u16) -> u16;
    /// Emit opcode ADDMOD
    pub fn mulmod_u16(a: u16, b: u16, n: u16) -> u16;

    /// Emit opcode ADDMOD
    pub fn addmod_u32(a: u32, b: u32, n: u32) -> u32;
    /// Emit opcode ADDMOD
    pub fn mulmod_u32(a: u32, b: u32, n: u32) -> u32;

    /// Emit opcode ADDMOD
    pub fn addmod_u64(a: u64, b: u64, n: u64) -> u64;
    /// Emit opcode ADDMOD
    pub fn mulmod_u64(a: u64, b: u64, n: u64) -> u64;

    /// Revert with message in 32 bytes
    pub fn revert1(message: &'static str);

    /// Revert with message in 64 bytes
    pub fn revert2(message: &'static str);

    /// Revert with message in 96 bytes
    pub fn revert3(message: &'static str);

    /// Revert with message in 128 bytes
    pub fn revert4(message: &'static str);

    /// Load a 8-bit signed integer from the storage.
    pub fn sload_i8() -> i8;

    /// Load a 8-bit unsigned integer from the storage.
    pub fn sload_u8() -> u8;

    /// Load a 16-bit signed integer from the storage.
    pub fn sload_i16() -> i16;

    /// Load a 16-bit unsigned integer from the storage.
    pub fn sload_u16() -> u16;

    /// Load a 32-bit signed integer from the storage.
    pub fn sload_i32() -> i32;

    /// Load a 32-bit unsigned integer from the storage.
    pub fn sload_u32() -> u32;

    /// Load a 64-bit signed integer from the storage.
    pub fn sload_i64() -> i64;

    /// Load a 64-bit unsigned integer from the storage.
    pub fn sload_u64() -> u64;

    /// Load a 8-bit signed integer from the transient storage.
    pub fn tload_i8() -> i8;

    /// Load a 8-bit unsigned integer from the transient storage.
    pub fn tload_u8() -> u8;

    /// Load a 16-bit signed integer from the transient storage.
    pub fn tload_i16() -> i16;

    /// Load a 16-bit unsigned integer from the transient storage.
    pub fn tload_u16() -> u16;

    /// Load a 32-bit signed integer from the transient storage.
    pub fn tload_i32() -> i32;

    /// Load a 32-bit unsigned integer from the transient storage.
    pub fn tload_u32() -> u32;

    /// Load a 64-bit signed integer from the transient storage.
    pub fn tload_i64() -> i64;

    /// Load a 64-bit unsigned integer from the transient storage.
    pub fn tload_u64() -> u64;

    /// Store a 8-bit signed integer to the transient storage.
    pub fn tstore_i8(val: i8);

    /// Store a 8-bit unsigned integer to the transient storage.
    pub fn tstore_u8(val: u8);

    /// Store a 16-bit signed integer to the transient storage.
    pub fn tstore_i16(val: i16);

    /// Store a 16-bit unsigned integer to the transient storage.
    pub fn tstore_u16(val: u16);

    /// Store a 32-bit signed integer to the transient storage.
    pub fn tstore_i32(val: i32);

    /// Store a 32-bit unsigned integer to the transient storage.
    pub fn tstore_u32(val: u32);

    /// Store a 64-bit signed integer to the transient storage.
    pub fn tstore_i64(val: i64);

    /// Store a 64-bit unsigned integer to the transient storage.
    pub fn tstore_u64(val: u64);
}

#[cfg(not(target_arch = "wasm32"))]
pub mod asm {
    pub fn push_i8(_val: i8) {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn push_u8(_val: u8) {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn push_i16(_val: i16) {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn push_u16(_val: u16) {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn push_i32(_val: i32) {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn push_u32(_val: u32) {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn push_i64(_val: i64) {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn push_u64(_val: u64) {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn addmod_i8(_a: i8, _b: i8, _n: i8) -> i8 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn mulmod_i8(_a: i8, _b: i8, _n: i8) -> i8 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn addmod_i16(_a: i16, _b: i16, _n: i16) -> i16 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn mulmod_i16(_a: i16, _b: i16, _n: i16) -> i16 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn addmod_i32(_a: i32, _b: i32, _n: i32) -> i32 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn mulmod_i32(_a: i32, _b: i32, _n: i32) -> i32 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn addmod_i64(_a: i64, _b: i64, _n: i64) -> i64 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn mulmod_i64(_a: i64, _b: i64, _n: i64) -> i64 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn addmod_u8(_a: u8, _b: u8, _n: u8) -> u8 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn mulmod_u8(_a: u8, _b: u8, _n: u8) -> u8 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn addmod_u16(_a: u16, _b: u16, _n: u16) -> u16 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn mulmod_u16(_a: u16, _b: u16, _n: u16) -> u16 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn addmod_u32(_a: u32, _b: u32, _n: u32) -> u32 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn mulmod_u32(_a: u32, _b: u32, _n: u32) -> u32 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn addmod_u64(_a: u64, _b: u64, _n: u64) -> u64 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn mulmod_u64(_a: u64, _b: u64, _n: u64) -> u64 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn revert1(message: &'static str) {
        panic!("Revert: {}", message);
    }

    pub fn revert2(_message: &'static str) {
        panic!("Revert called");
    }

    pub fn revert3(_message: &'static str) {
        panic!("Revert called");
    }

    pub fn revert4(_message: &'static str) {
        panic!("Revert called");
    }

    pub fn sload(_slot: i32) -> i32 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn sload_i8() -> i8 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn sload_u8() -> u8 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn sload_i16() -> i16 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn sload_u16() -> u16 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn sload_i32() -> i32 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn sload_u32() -> u32 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn sload_i64() -> i64 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn sload_u64() -> u64 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn sstore(_slot: i32, _value: i32) {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn tload(_slot: i32) -> i32 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn tload_i8() -> i8 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn tload_u8() -> u8 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn tload_i16() -> i16 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn tload_u16() -> u16 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn tload_i32() -> i32 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn tload_u32() -> u32 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn tload_i64() -> i64 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn tload_u64() -> u64 {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn tstore(_slot: i32, _value: i32) {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn tstore_i8(_val: i8) {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn tstore_u8(_val: u8) {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn tstore_i16(_val: i16) {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn tstore_u16(_val: i16) {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn tstore_i32(_val: i32) {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn tstore_u32(_val: u32) {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn tstore_i64(_val: i64) {
        unimplemented!("Only available in wasm32 target");
    }

    pub fn tstore_u64(_val: u64) {
        unimplemented!("Only available in wasm32 target");
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use asm::*;
