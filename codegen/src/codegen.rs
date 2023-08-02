//! Code generation implementation.
use crate::{
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

        let mut codegen = Self {
            control: ControlStack::default(),
            env,
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
            codegen.masm.shift_pc(params_count, true)?;
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
        // Define locals in function parameters.
        for (idx, param) in self.env.params().iter().enumerate() {
            let sp = if self.is_main {
                None
            } else {
                // NOTE: no need to increment sp here bcz we have
                // already did it in the constructor.
                Some(idx + 1)
            };

            self.locals
                .push(LocalSlot::new(*param, LocalSlotType::Parameter, sp));
        }

        // Define locals in function body.
        //
        // Record the offset for validation.
        let params_len = self.env.params().len();
        while let Ok((count, val)) = locals.read() {
            let sp = if self.is_main {
                None
            } else {
                Some(params_len + 1)
            };

            let validation_offset = locals.original_position();
            for offset in 0..count {
                // Init locals with zero.
                self.masm.push(&[0])?;

                // Define locals.
                self.locals.push(LocalSlot::new(
                    val,
                    LocalSlotType::Variable,
                    sp.map(|sp| sp + offset as usize),
                ));
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
