//! Code generation library for zink.
#![deny(missing_docs)]
#![recursion_limit = "1024"]

pub use crate::{
    abi::{ToLSBytes, Type},
    asm::Assembler,
    code::Code,
    codegen::CodeGen,
    constructor::Constructor,
    control::{ControlStack, ControlStackFrame, ControlStackFrameType},
    data::DataSet,
    dispatcher::Dispatcher,
    func::{Function, Functions},
    jump::JumpTable,
    local::{LocalSlot, Locals},
    masm::MacroAssembler,
    result::{Error, Result},
    wasm::{Exports, HostFunc, Imports},
};
use smallvec::SmallVec;

pub mod abi;
mod asm;
mod backtrace;
mod code;
mod codegen;
mod constructor;
mod control;
mod data;
mod dispatcher;
mod func;
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
