//! Code generation implementation.
use crate::{
    abi::Type, local::LocalSlot, masm::MacroAssembler, validator::ValidateThenVisit, Result,
};
use smallvec::SmallVec;
use wasmparser::{FuncType, FuncValidator, LocalsReader, OperatorsReader, ValidatorResources};

/// The code generation abstraction.
///
/// TODO: add codegen context for backtrace. (#21)
pub struct CodeGen {
    /// The macro assembler.
    pub masm: MacroAssembler,
    /// The defined locals for a function.
    ///
    /// NOTE: Solidity's implementation uses 16 slots for locals.
    ///
    /// ref: https://docs.soliditylang.org/en/v0.8.20/internals/optimizer.html#stackcompressor
    pub locals: SmallVec<[LocalSlot; 16]>,
    /// The function environment.
    pub env: FuncType,
}

impl CodeGen {
    /// Create a new code generator.
    pub fn new(env: FuncType) -> Self {
        Self {
            env,
            locals: SmallVec::new(),
            masm: MacroAssembler::default(),
        }
    }

    /// Get the generated buffer.
    pub fn buffer(&self) -> &[u8] {
        self.masm.buffer()
    }

    /// Emit function locals
    ///
    /// 1. the function parameters.
    /// 2. function body locals.
    pub fn emit_locals<'a>(
        &mut self,
        locals: &mut LocalsReader<'a>,
        validator: &mut FuncValidator<ValidatorResources>,
    ) -> Result<()> {
        let mut offset = 0;

        // Define locals in function parameters.
        for param in self.env.params() {
            self.locals.push(LocalSlot::new(offset, *param));
            offset += param.align();
        }

        // Define locals in function body.
        //
        // Record the offset for validation.
        while let Ok((count, val)) = locals.read() {
            let validation_offset = locals.original_position();
            let slot = LocalSlot::new(offset, val);
            let size = slot.size() as usize;

            self.locals.push(slot);
            validator.define_locals(validation_offset, count, val)?;
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
            let mut validate_then_visit = ValidateThenVisit(validator.visitor(offset), self);
            ops.visit_operator(&mut validate_then_visit)???;
        }

        Ok(())
    }
}
