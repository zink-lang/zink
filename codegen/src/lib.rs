//! Code generation library for zink.
#![deny(missing_docs)]
#![recursion_limit = "1024"]

pub use crate::{
    abi::{ToLSBytes, Type},
    asm::Assembler,
    codegen::CodeGen,
    control::{ControlStack, ControlStackFrame, ControlStackFrameType},
    jump::JumpTable,
    local::LocalSlot,
    masm::MacroAssembler,
    result::{Error, Result},
};
use smallvec::SmallVec;

pub mod abi;
mod asm;
mod codegen;
mod control;
mod jump;
mod local;
mod masm;
mod result;
mod validator;
mod visitor;

/// Maximum size of a evm bytecode in bytes.
pub const BUFFER_LIMIT: usize = 0x6000;

/// Code generation buffer.
pub type Buffer = SmallVec<[u8; BUFFER_LIMIT]>;

/// Solidity's implementation uses 16 slots for locals.
/// ref: <https://docs.soliditylang.org/en/v0.8.20/internals/optimizer.html#stackcompressor>
pub type Locals = SmallVec<[LocalSlot; 16]>;
