//! Code generation library for zink.
#![deny(missing_docs)]
#![recursion_limit = "1024"]

pub use crate::{
    abi::Type,
    asm::Assembler,
    codegen::CodeGen,
    control::{ControlStack, ControlStackFrame, ControlStackFrameType},
    local::LocalSlot,
    masm::MacroAssembler,
    result::{Error, Result},
};

mod abi;
mod asm;
mod codegen;
mod control;
mod local;
mod masm;
mod result;
mod validator;
mod visitor;
