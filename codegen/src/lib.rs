//! Code generation library for zink.
#![deny(missing_docs)]
#![recursion_limit = "1024"]

use crate::parser::ValidateThenVisit;
pub use crate::{
    abi::LocalSlot,
    asm::Assmbler,
    context::Context,
    frame::Frame,
    masm::MacroAssembler,
    result::{Error, Result},
    stack::Stack,
};
use wasmparser::{
    BinaryReader, FuncType, FuncValidator, LocalsReader, OperatorsReader, ValidatorResources,
};

mod abi;
mod asm;
mod context;
mod frame;
mod limits;
mod local;
mod masm;
mod parser;
mod result;
mod stack;
mod visitor;

/// The code generation abstraction.
///
/// TODO: add codegen context for backtrace.
pub struct CodeGen {
    masm: MacroAssembler,
}

impl CodeGen {
    /// Create a new code generator.
    pub fn new() -> Self {
        Self {
            masm: MacroAssembler::default(),
        }
    }

    /// Get the generated code.
    pub fn buffer(&self) -> &[u8] {
        self.masm.buffer()
    }

    // /// Emit function locals
    // pub fn emit_locals<'a>(
    //     &mut self,
    //     sig: FuncType,
    //     locals: &mut LocalsReader<'a>,
    //     validator: &mut FuncValidator<ValidatorResources>,
    // ) -> Result<()> {
    //     // while !locals.eof() {
    //     //
    //     // }
    //
    //     Ok(())
    // }

    /// Emit function operators
    pub fn emit_operators<'a>(
        &mut self,
        ops: &mut OperatorsReader<'a>,
        validator: &mut FuncValidator<ValidatorResources>,
    ) -> Result<()> {
        while !ops.eof() {
            let offset = ops.original_position();
            let _ = ops.visit_operator(&mut ValidateThenVisit(validator.visitor(offset), self))?;
        }

        Ok(())
    }
}
