//! Code generation library for zink.
#![deny(missing_docs)]
#![recursion_limit = "1024"]

pub use crate::{
    asm::Assembler,
    codegen::{Code, Constructor, Dispatcher, Function},
    control::{ControlStack, ControlStackFrame, ControlStackFrameType},
    jump::JumpTable,
    local::{LocalSlot, Locals},
    masm::MacroAssembler,
    result::{Error, Result},
};
use smallvec::SmallVec;

mod asm;
mod backtrace;
mod codegen;
mod control;
mod jump;
mod local;
mod masm;
mod result;
mod validator;
mod visitor;
pub mod wasm;

/// Maximum size of a evm bytecode in bytes.
pub const BUFFER_LIMIT: usize = 0x6000;

/// Code generation buffer.
pub type Buffer = SmallVec<[u8; BUFFER_LIMIT]>;
