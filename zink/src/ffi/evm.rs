//! EVM FFI.

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
    pub fn push32(val: i32);

    /// Store a value in the storage
    pub fn sstore(key: i32);

    /// Load a value from the storage
    pub fn sload(key: i32) -> *const i32;

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
