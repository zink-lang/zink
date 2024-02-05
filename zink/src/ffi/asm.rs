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
}
