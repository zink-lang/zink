//! Code generation library for zink.
#![deny(missing_docs)]
#![recursion_limit = "1024"]

pub use crate::{
    asm::Assembler,
    code::Code,
    codegen::CodeGen,
    constructor::Constructor,
    control::{ControlStack, ControlStackFrame, ControlStackFrameType},
    dispatcher::Dispatcher,
    jump::JumpTable,
    local::{LocalSlot, Locals},
    masm::MacroAssembler,
    result::{Error, Result},
    wasm::{Data, Exports, Function, Functions, HostFunc, Imports, ToLSBytes, Type},
};
use smallvec::SmallVec;

mod asm;
mod backtrace;
mod code;
mod codegen;
mod constructor;
mod control;
mod dispatcher;
mod jump;
mod local;
mod masm;
mod result;
mod validator;
mod visitor;
mod wasm;

/// Maximum size of a evm bytecode in bytes.
pub const BUFFER_LIMIT: usize = 0x6000;

/// Code generation buffer.
pub type Buffer = SmallVec<[u8; BUFFER_LIMIT]>;
