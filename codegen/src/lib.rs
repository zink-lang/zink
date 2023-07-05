//! Code generation library for zink.
#![deny(missing_docs)]

pub use crate::{
    asm::Assmbler,
    masm::MacroAssembler,
    result::{Error, Result},
};

mod asm;
mod limits;
mod masm;
mod result;
mod visitor;

/// The code generation abstraction.
#[derive(Default)]
pub struct CodeGen {
    _asm: MacroAssembler,
}
