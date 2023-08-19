//! System instructions

use crate::{CodeGen, Error, Func, Result};

impl CodeGen {
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
        // record the current program counter and
        // pass it to the callee function.
        self.masm._pc()?;

        // TODO: check the safty of the function index.
        let base = self.imports.len() as u32;

        if base > index {
            self.call_imported(index)
        } else {
            self.call_internal(base + index)
        }
    }

    /// Call internal functions
    fn call_internal(&mut self, index: u32) -> Result<()> {
        // Call an internal function.
        //
        // register the call index to the jump table.
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
            .imports
            .get(index as usize)
            .ok_or(Error::ImportedFuncNotFound(index))?;

        tracing::debug!("call imported function {func:?} at index {index}");
        match func {
            Func::Sstore => self.masm._swap2()?,
            Func::Sload => self.masm._swap1()?,
            _ => {}
        }

        self.table.ext(self.masm.pc_offset(), func);

        self.masm._jump()?;
        self.masm._jumpdest()?;
        Ok(())
    }
}
