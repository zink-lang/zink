//! Codegen results

/// Codegen error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to parse WASM with binary reader.
    #[error(transparent)]
    BinaryReader(#[from] wasmparser::BinaryReaderError),
    /// Failed to push more data to the buffer.
    #[error("Buffer overflow: {0}, the limit of the binary buffer is 0x6000.")]
    BufferOverflow(usize),
    /// Failed to define local variable since the index is out of range.
    #[error("Local index in function is out of range")]
    LocalIndexOutOfRange,
    /// Failed to index data on stack.
    #[error("Stack index is out of range {0}, max is 32 (0x400)")]
    StackIndexOutOfRange(u8),
}

/// Codegen result
pub type Result<T> = std::result::Result<T, Error>;
