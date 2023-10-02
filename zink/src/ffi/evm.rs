//! EVM imports

// EVM interfaces
//
// TODO: Align to 256-bit #20.
#[link(wasm_import_module = "evm")]
#[allow(improper_ctypes)]
extern "C" {
    // i32 -> 8 bytes

    /// Store a value in the storage
    pub fn sstore(key: i32, value: i32);

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

    /// Copy code running in current environment to memory
    pub fn codecopy(destOffset: u32, codeOffset: u32, size: u32);
}
