//! Assembly FFI.

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
