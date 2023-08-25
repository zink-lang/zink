//! EVM imports

// EVM interfaces
//
// TODO: Align to 256-bit #20.
#[link(wasm_import_module = "evm")]
extern "C" {
    // i64 -> 8 bytes

    /// Store a value in the storage
    pub fn sstore(
        // Storage key
        key: i64,
        // value key
        value: i64,
    );

    /// Load a value from the storage
    pub fn sload(key: i64) -> i64;

    /// Append log record with no topics
    pub fn log0(offset: i64, size: i64);

    /// Append log record with one topics
    pub fn log1(offset: i64, size: i64);

    /// Append log record with two topics
    pub fn log2(offset: i64, size: i64);

    /// Append log record with three topics
    pub fn log3(offset: i64, size: i64);

    /// Append log record with four topics
    pub fn log4(offset: i64, size: i64);
}
