//! EVM imports

// EVM interfaces
//
// TODO: Align to 256-bit #20.
#[link(wasm_import_module = "evm")]
extern "C" {
    // i32 -> 8 bytes

    /// Store a value in the storage
    pub fn sstore(key: i32, value: i32);

    /// Load a value from the storage
    pub fn sload(key: i32) -> i32;

    /// Append log record with no topics
    ///
    /// NOTE:
    ///
    /// the first argument of this function is the position
    /// of the data in the data section.
    pub fn log0(offset: i32, size: i32);

    /// Append log record with one topics
    pub fn log1(offset: i32, size: i32);

    /// Append log record with two topics
    pub fn log2(offset: i32, size: i32);

    /// Append log record with three topics
    pub fn log3(offset: i32, size: i32);

    /// Append log record with four topics
    pub fn log4(offset: i32, size: i32);

    /// Copy code running in current environment to memory
    pub fn codecopy(destOffset: u32, codeOffset: u32, size: u32);

    // TODO: introduce inline assmebly (#115)
}
