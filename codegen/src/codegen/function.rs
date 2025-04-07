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
use opcodes::ShangHai as OpCode;
use wasmparser::{FuncType, FuncValidator, LocalsReader, OperatorsReader, ValidatorResources};
use zabi::Abi;

/// The code generation abstraction.
pub struct Function {
    /// Abi of this function,
    pub abi: Option<Abi>,
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
    ///
    pub takes: u16,
    ///
    pub returns: u16,
}

impl Function {
    /// Create a new code generator.
    pub fn new(env: Env, ty: FuncType, abi: Option<Abi>, is_main: bool) -> Result<Self> {
        let takes = ty.params().len() as u16;
        let returns = ty.results().len() as u16;
        let is_external = abi.is_some();
        let mut codegen = Self {
            abi,
            backtrace: Backtrace::default(),
            control: ControlStack::default(),
            env,
            ty,
            locals: Default::default(),
            masm: MacroAssembler::default(),
            table: Default::default(),
            is_main,
            takes,
            returns,
        };

        if is_main || is_external {
            codegen.masm.set_sp(0)?; // Params loaded via CALLDATALOAD
        } else {
            codegen.masm.set_sp(1)?; // Return PC for internal calls
        }

        if !is_main && !is_external {
            codegen.masm._jumpdest()?;
        }

        Ok(codegen)
    }

    /// Emit function locals
    ///
    /// 1. the function parameters.
    /// 2. function body locals.
    ///
    /// NOTE: we don't care about the original offset of the locals.
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
            for _ in 0..count {
                // Define locals.
                self.locals
                    .push(LocalSlot::new(val, LocalSlotType::Variable, sp));

                sp += 1;
            }

            let validation_offset = locals.original_position();
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

        if (self.abi.is_some() || self.is_main)
            && self.masm.buffer().last() != Some(&OpCode::RETURN.into())
        {
            self._end()?;
        }

        Ok(())
    }

    /// Finish code generation.
    pub fn finish(self, jump_table: &mut JumpTable, pc: u16) -> Result<Buffer> {
        // Remove SP check here; trust _end's validation
        jump_table.merge(self.table, pc)?;
        Ok(self.masm.buffer().into())
    }
}
