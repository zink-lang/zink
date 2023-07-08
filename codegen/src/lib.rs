//! Code generation library for zink.
#![deny(missing_docs)]
#![recursion_limit = "1024"]

pub use crate::{
    abi::LocalSlot,
    asm::Assembler,
    codegen::CodeGen,
    masm::MacroAssembler,
    result::{Error, Result},
    stack::Stack,
};

mod abi;
mod asm;
mod codegen;
mod masm;
mod parser;
mod result;
mod stack;
mod visitor;
