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
    /// Failed to find ext function index in jump table.
    #[error("External function {0:?} not found in jump table")]
    ExtNotFound(crate::Func),
    /// Failed to find function index in jump table.
    #[error("Function {0} not found in jump table")]
    FuncNotFound(u32),
    /// Failed to find host function in compiler.
    #[error("Host function {0}::{1} not found in compiler")]
    HostFuncNotFound(String, String),
    /// Failed to find imported function by index in jump table.
    #[error("Imported Function {0} not found in jump table")]
    ImportedFuncNotFound(u32),
    /// Failed to mark else block for if block.
    #[error("Invalid else block for if block at {0}")]
    InvalidElseBlock(u16),
    /// Failed to get local with given index.
    #[error("Invalid local index {0}")]
    InvalidLocalIndex(usize),
    /// Failed to get the offset of the given memory pointer.
    #[error("Invalid memory pointer {0}")]
    InvalidMP(u8),
    /// Failed to construct program counter for jump.
    #[error("Invalid program counter {0}")]
    InvalidPC(usize),
    /// Failed to get frame info of the given depth.
    #[error("Invalid contract stack fram depth {0}")]
    InvalidDepth(usize),
    /// Failed to patch jump destination.
    #[error("Invalid frame label")]
    LabelMismatch,
    /// Failed to parse WASM with leb128 reader.
    #[error(transparent)]
    Leb128(#[from] leb128::read::Error),
    /// Failed to define local variable since the index is out of range.
    #[error("Local index in function is out of range")]
    LocalIndexOutOfRange,
    /// Failed to get local variables.
    #[error("Local variable {0} is not on stack")]
    LocalNotOnStack(usize),
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
    /// Failed to pop stack.
    #[error("Stack not balanced, current stack items {0}")]
    StackNotBalanced(u8),
    /// Failed to queue host functions.
    #[error("Unsupported host function {0:?}")]
    UnsupportedHostFunc(crate::Func),
}

/// Codegen result
pub type Result<T> = std::result::Result<T, Error>;
