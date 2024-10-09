//! call instructions

use crate::{wasm::HostFunc, Error, Function, Result};
use opcodes::ShangHai as OpCode;

impl Function {
    /// The call indirect instruction calls a function indirectly
    /// through an operand indexing into a table.
    pub fn _call_indirect(
        &mut self,
        _type_index: u32,
        _table_index: u32,
        _table_byte: u8,
    ) -> Result<()> {
        todo!()
    }

    /// The call instruction calls a function specified by its index.
    pub fn _call(&mut self, index: u32) -> Result<()> {
        if self.env.is_external(index) {
            // TODO: throw with error
            panic!("External functions could not be called internally");
        }

        if self.env.imports.len() as u32 > index {
            self.call_imported(index)
        } else {
            self.call_internal(index)
        }
    }

    /// Call internal functions
    fn call_internal(&mut self, index: u32) -> Result<()> {
        tracing::trace!("call internal function: index={index}");
        // record the current program counter and
        // pass it to the callee function.
        self.table.offset(self.masm.pc_offset(), 6);
        self.masm.increment_sp(1)?;
        self.masm._jumpdest()?;

        // Call an internal function.
        //
        // register the call index to the jump table.
        //
        // TODO: support same pc different jumps. (#160)
        self.table.call(self.masm.pc_offset(), index);

        // jump to the callee function
        //
        // TODO: check the stack output.
        self.masm._jump()?;
        self.masm._jumpdest()?;

        Ok(())
    }

    /// Call imported functions
    fn call_imported(&mut self, index: u32) -> Result<()> {
        // call an imported function.
        //
        // register the imported function index to the jump table.
        let func = *self
            .env
            .imports
            .get(&index)
            .ok_or(Error::ImportedFuncNotFound(index))?;

        tracing::trace!("call imported function, index={index}, func={func:?}");

        match func {
            HostFunc::Evm(OpCode::LOG0) => self.log(0),
            HostFunc::Evm(OpCode::LOG1) => self.log(1),
            HostFunc::Evm(OpCode::LOG2) => self.log(2),
            HostFunc::Evm(OpCode::LOG3) => self.log(3),
            HostFunc::Evm(OpCode::LOG4) => self.log(4),
            HostFunc::Evm(op) => self.masm.emit_op(op),
            HostFunc::NoOp | HostFunc::Label(_) => Ok(()),
            _ => {
                tracing::error!("unsupported host function {func:?}");
                Err(Error::UnsupportedHostFunc(func))
            }
        }
    }
}
