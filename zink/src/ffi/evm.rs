//! EVM FFI.

#[link(wasm_import_module = "evm")]
#[allow(improper_ctypes)]
extern "C" {
    /// Push 1 byte to the stack.
    pub fn push0();

    /// Push 1 byte to the stack.
    pub fn push1(val: u8);

    /// Push 2 bytes to the stack.
    pub fn push2(val: u8);

    /// Push 3 bytes to the stack.
    pub fn push3(val: u8);

    /// Push 4 bytes to the stack.
    pub fn push4(val: u8);

    /// Push 5 bytes to the stack.
    pub fn push5(val: u8);

    /// Push 6 bytes to the stack.
    pub fn push6(val: u8);

    /// Push 7 bytes to the stack.
    pub fn push7(val: u8);

    /// Push 8 bytes to the stack.
    pub fn push8(val: u8);

    /// Push 9 bytes to the stack.
    pub fn push9(val: u8);

    /// Push 10 bytes to the stack.
    pub fn push10(val: u8);

    /// Push 11 bytes to the stack.
    pub fn push11(val: u8);

    /// Push 12 bytes to the stack.
    pub fn push12(val: u8);

    /// Push 13 bytes to the stack.
    pub fn push13(val: u8);

    /// Push 14 bytes to the stack.
    pub fn push14(val: u8);

    /// Push 15 bytes to the stack.
    pub fn push15(val: u8);

    /// Push 16 bytes to the stack.
    pub fn push16(val: u8);

    /// Push 17 bytes to the stack.
    pub fn push17(val: u8);

    /// Push 18 bytes to the stack.
    pub fn push18(val: u8);

    /// Push 19 bytes to the stack.
    pub fn push19(val: u8);

    /// Push 20 bytes to the stack.
    pub fn push20(val: u8);

    /// Push 21 bytes to the stack.
    pub fn push21(val: u8);

    /// Push 22 bytes to the stack.
    pub fn push22(val: u8);

    /// Push 23 bytes to the stack.
    pub fn push23(val: u8);

    /// Push 24 bytes to the stack.
    pub fn push24(val: u8);

    /// Push 25 bytes to the stack.
    pub fn push25(val: u8);

    /// Push 26 bytes to the stack.
    pub fn push26(val: u8);

    /// Push 27 bytes to the stack.
    pub fn push27(val: u8);

    /// Push 28 bytes to the stack.
    pub fn push28(val: u8);

    /// Push 29 bytes to the stack.
    pub fn push29(val: u8);

    /// Push 30 bytes to the stack.
    pub fn push30(val: u8);

    /// Push 31 bytes to the stack.
    pub fn push31(val: u8);

    /// Push 32 bytes to the stack.
    pub fn push32(val: u8);

    /// Store a value in the storage
    pub fn sstore(value: i32, key: i32);

    /// Load a value from the storage
    pub fn sload(key: i32) -> i32;

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
