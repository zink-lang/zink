//! Code generation implementation.
use crate::{
    backtrace::Backtrace,
    control::ControlStack,
    jump::JumpTable,
    local::{LocalSlot, LocalSlotType, Locals},
    masm::MacroAssembler,
    validator::ValidateThenVisit,
    wasm::Env,
    Buffer, Result,
};
use wasmparser::{FuncType, FuncValidator, LocalsReader, OperatorsReader, ValidatorResources};

/// The code generation abstraction.
pub struct Function {
    /// The backtrace.
    pub backtrace: Backtrace,
    /// Control stack frames.
    pub control: ControlStack,
    /// WASM environment.
    pub env: Env,
    /// The defined locals for a function.
    pub locals: Locals,
    /// The macro assembler.
    pub masm: MacroAssembler,
    /// The jump table.
    pub table: JumpTable,
    /// The function type.
    pub ty: FuncType,
    /// If this function is the main function.
    pub is_main: bool,
}

impl Function {
    /// Create a new code generator.
    pub fn new(env: Env, ty: FuncType, is_main: bool) -> Result<Self> {
        let mut params_count = 0;
        if !is_main {
            params_count = ty.params().len() as u8;
        }

        let mut codegen = Self {
            backtrace: Backtrace::default(),
            control: ControlStack::default(),
            env,
            ty,
            locals: Default::default(),
            masm: Default::default(),
            table: Default::default(),
            is_main,
        };

        // post process program counter and stack pointer.
        if !is_main {
            // Mock the stack frame for the callee function
            //
            // STACK: PC + params
            codegen.masm.increment_sp(1 + params_count)?;
            codegen.masm._jumpdest()?;
            codegen.masm.shift_stack(params_count, true)?;
        }

        Ok(codegen)
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
        let mut sp = if self.is_main { 0 } else { 1 };

        // Define locals in function parameters.
        for param in self.ty.params() {
            self.locals
                .push(LocalSlot::new(*param, LocalSlotType::Parameter, sp));
            sp += 1;
        }

        // Define locals in function body.
        //
        // Record the offset for validation.
        while let Ok((count, val)) = locals.read() {
            let validation_offset = locals.original_position();
            for _ in 0..count {
                // Init locals with zero.
                self.masm.push(&[0])?;

                // Define locals.
                self.locals
                    .push(LocalSlot::new(val, LocalSlotType::Variable, sp));

                sp += 1;
            }

            validator.define_locals(validation_offset, count, val)?;
        }

        tracing::trace!("{:?}", self.locals);
        Ok(())
    }

    /// Emit function operators.
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
        // TODO: check the stack here
        //
        // let sp = self.masm.sp();
        // if !self.is_main && self.masm.sp() != self.ty.results().len() as u8 {
        //     return Err(Error::StackNotBalanced(sp));
        // }

        jump_table.merge(self.table, pc)?;
        Ok(self.masm.buffer().into())
    }
}
