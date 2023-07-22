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
    /// Failed to pop control stack frame.
    #[error("Control stack underflow")]
    ControlStackUnderflow,
    /// Failed to register program counter to function index.
    #[error("Function {0} already exists in jump table")]
    DuplicateFunc(u32),
    /// Failed to merge jump table.
    #[error("Program counter {0} already exists in jump table")]
    DuplicateJump(u16),
    /// Failed to find function index in jump table.
    #[error("Function {0} not found in jump table")]
    FuncNotFound(u32),
    /// Failed to construct program counter for jump.
    #[error("Invalid program counter {0}")]
    InvalidPC(usize),
    /// Failed to get frame info of the given depth.
    #[error("Invalid contract stack fram depth {0}")]
    InvalidDepth(usize),
    /// Failed to patch jump destination.
    #[error("Invalid frame label")]
    LabelMismatch,
    /// Failed to define local variable since the index is out of range.
    #[error("Local index in function is out of range")]
    LocalIndexOutOfRange,
    /// Failed to index data on memory.
    #[error("Memory index is out of range")]
    MemoryOutOfBounds,
    /// Failed to index data on stack.
    #[error("Stack index is out of range {0}, max is 32 (0x400)")]
    StackIndexOutOfRange(u8),
    /// Failed to increment stack pointer.
    #[error("Stack overflow, max is 12 stack items, got {0}")]
    StackOverflow(u8),
    /// Failed to decrement stack pointer.
    #[error("Stack underflow, current stack items {0}, expect at least {1}")]
    StackUnderflow(u8, u8),
}

/// Codegen result
pub type Result<T> = std::result::Result<T, Error>;
