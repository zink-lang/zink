//! Code generation implementation.
use crate::{
    abi::Type,
    control::ControlStack,
    jump::JumpTable,
    local::{LocalSlot, LocalSlotType, Locals},
    masm::MacroAssembler,
    validator::ValidateThenVisit,
    Buffer, Result,
};
use wasmparser::{FuncType, FuncValidator, LocalsReader, OperatorsReader, ValidatorResources};

/// The code generation abstraction.
///
/// TODO: add codegen context for backtrace. (#21)
pub struct CodeGen {
    /// Control stack frames.
    pub(crate) control: ControlStack,
    /// The function environment.
    pub(crate) env: FuncType,
    /// The defined locals for a function.
    pub(crate) locals: Locals,
    /// The macro assembler.
    pub(crate) masm: MacroAssembler,
    /// The jump table.
    pub(crate) table: JumpTable,
    /// If this function is the main function.
    pub(crate) is_main: bool,
}

impl CodeGen {
    /// Create a new code generator.
    pub fn new(env: FuncType, is_main: bool) -> Result<Self> {
        let mut params_count = 0;
        if !is_main {
            params_count = env.params().len() as u8;
        }

        Ok(Self {
            control: ControlStack::default(),
            env,
            locals: Default::default(),
            masm: MacroAssembler::new(is_main, params_count)?,
            table: Default::default(),
            is_main,
        })
    }

    /// Emit function locals
    ///
    /// 1. the function parameters.
    /// 2. function body locals.
    ///
    /// NOTE: we don't care about the origin offset of the locals.
    /// bcz we will serialize the locals to an index map anyway.
    pub fn emit_locals(
        &mut self,
        locals: &mut LocalsReader<'_>,
        validator: &mut FuncValidator<ValidatorResources>,
    ) -> Result<()> {
        // Define locals in function parameters.
        for param in self.env.params() {
            self.locals
                .push(LocalSlot::new(*param, LocalSlotType::Parameter));
        }

        // Define locals in function body.
        //
        // Record the offset for validation.
        while let Ok((count, val)) = locals.read() {
            let validation_offset = locals.original_position();
            for _ in 0..count {
                self.locals
                    .push(LocalSlot::new(val, LocalSlotType::Variable));
                self.masm.increment_mp(val.align())?;
            }

            validator.define_locals(validation_offset, count, val)?;
        }

        tracing::debug!("{:?}", self.locals);

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

    /// Finish code generation.
    pub fn finish(self, jump_table: &mut JumpTable, pc: u16) -> Result<Buffer> {
        jump_table.merge(self.table, pc)?;
        Ok(self.masm.buffer().into())
    }
}
