//! WASM module imports

// Zink provided interfaces
#[link(wasm_import_module = "zink")]
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
}
