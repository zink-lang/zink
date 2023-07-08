//! Code generation implementation.
use crate::{parser::ValidateThenVisit, LocalSlot, MacroAssembler, Result};
use smallvec::SmallVec;
use wasmparser::{FuncType, FuncValidator, LocalsReader, OperatorsReader, ValidatorResources};

/// The code generation abstraction.
///
/// TODO: add codegen context for backtrace. (#21)
#[derive(Default)]
pub struct CodeGen {
    /// The macro assembler.
    pub masm: MacroAssembler,
    /// The defined locals for a function.
    ///
    /// NOTE: Solidity's implementation uses 16 slots for locals.
    ///
    /// ref: https://docs.soliditylang.org/en/v0.8.20/internals/optimizer.html#stackcompressor
    pub locals: SmallVec<[LocalSlot; 16]>,
}

impl CodeGen {
    /// Create a new code generator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the generated code.
    pub fn buffer(&self) -> &[u8] {
        self.masm.buffer()
    }

    /// Emit function locals
    ///
    /// 1. the function parameters.
    /// 2. function body locals.
    /// 3. the function return value.
    pub fn emit_locals<'a>(&mut self, sig: FuncType, locals: &mut LocalsReader<'a>) -> Result<()> {
        let mut offset = 0;

        while let Ok((_, val)) = locals.read() {
            let slot = LocalSlot::new(val, offset);
            let size = slot.size();
            self.locals.push(slot);
            offset += size;
        }

        Ok(())
    }

    /// Emit function operators
    pub fn emit_operators(
        &mut self,
        ops: &mut OperatorsReader<'_>,
        validator: &mut FuncValidator<ValidatorResources>,
    ) -> Result<()> {
        while !ops.eof() {
            let offset = ops.original_position();
            let _ = ops.visit_operator(&mut ValidateThenVisit(validator.visitor(offset), self))?;
        }

        Ok(())
    }
}
