//! Code generation library for zink.
#![deny(missing_docs)]
#![recursion_limit = "1024"]

pub use crate::{
    abi::Type,
    asm::Assembler,
    codegen::CodeGen,
    local::LocalSlot,
    masm::MacroAssembler,
    result::{Error, Result},
    stack::Stack,
};

mod abi;
mod asm;
mod codegen;
mod local;
mod masm;
mod result;
mod stack;
mod validator;
mod visitor;
