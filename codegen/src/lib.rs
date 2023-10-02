//! Code generation library for zink.
#![deny(missing_docs)]
#![recursion_limit = "1024"]

pub use crate::{
    abi::{ToLSBytes, Type},
    asm::Assembler,
    codegen::CodeGen,
    control::{ControlStack, ControlStackFrame, ControlStackFrameType},
    data::DataSet,
    func::Func,
    jump::{Code, JumpTable},
    local::{LocalSlot, Locals},
    masm::MacroAssembler,
    result::{Error, Result},
};
use smallvec::SmallVec;

pub mod abi;
mod asm;
mod backtrace;
mod codegen;
mod control;
mod data;
mod func;
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

/// Imported functions.
/// pub type Imports = IndexMap<u32, Func>;
pub type Imports = Vec<Func>;
